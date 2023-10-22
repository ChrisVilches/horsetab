use crate::{
  cmd_parser::{parse_cmd, Cmd},
  logger::{log_stderr, log_stdout},
};
use anyhow::Result;
use chrono::{DateTime, Local};
use std::{
  fs::File,
  io::{BufRead, BufReader},
  process::{Command, Stdio},
};

fn parse_lines(lines: Vec<std::io::Result<String>>) -> Result<Vec<Cmd>> {
  let commands = lines
    .into_iter()
    .map(|line| parse_cmd(&line?))
    .collect::<Result<Vec<Option<Cmd>>>>()?;

  Ok(commands.into_iter().flatten().collect())
}

pub fn read_commands(file_path: &str) -> Result<Vec<Cmd>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  parse_lines(reader.lines().collect())
}

fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

pub fn spawn_process(cmd: &str) {
  let start_time = Local::now();

  let mut process = Command::new("bash")
    .arg("-c")
    .arg(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Should execute command");

  let status = process.wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  let stdout = process.stdout.unwrap();
  let stderr = process.stderr.unwrap();

  log_stdout(stdout, cmd, status, start_time, elapsed_sec);
  log_stderr(stderr, cmd, status, start_time, elapsed_sec);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn build_lines(lines: Vec<&str>) -> Vec<std::io::Result<String>> {
    lines
      .into_iter()
      .map(|s| std::io::Result::Ok(s.to_owned()))
      .collect()
  }

  #[test]
  fn test_read_multiple_empty_lines() {
    let cmd = parse_lines(build_lines(vec![" ", "   ", " "]));
    assert!(cmd.is_ok());
    assert!(cmd.unwrap().is_empty());
  }

  #[test]
  fn test_read_wrong_lines() {
    let cmd = parse_lines(build_lines(vec![" ", "  x ", " "]));
    assert!(cmd.is_err());
    assert_eq!(
      cmd.err().unwrap().to_string(),
      "Line should contain at least one space"
    );
  }

  #[test]
  fn test_read_commands_ok() {
    let result = parse_lines(build_lines(vec![" .- x ", " . xyz ", " .--  yyy"]));
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd.len(), 3);
    assert_eq!(cmd[0].sequence, "01");
    assert_eq!(cmd[1].sequence, "0");
    assert_eq!(cmd[2].sequence, "011");
    assert_eq!(cmd[0].command, "x");
    assert_eq!(cmd[1].command, "xyz");
    assert_eq!(cmd[2].command, "yyy");
  }
}
