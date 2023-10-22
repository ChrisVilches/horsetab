use crate::{
  cmd_parser::{parse_cmd, Cmd},
  logger::{log_stderr, log_stdout},
};
use chrono::{DateTime, Utc};
use std::{
  error::Error,
  fs::File,
  io::{BufRead, BufReader},
  process::{Command, Stdio},
};

pub fn read_commands(file_path: &str) -> Result<Vec<Cmd>, Box<dyn Error>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);

  // TODO: I don't know if this silences errors (with the filter_map).
  //       But anyway I have to test the failure more extensively later.
  Ok(
    reader
      .lines()
      .flatten()
      .filter_map(|l| parse_cmd(&l))
      .collect(),
  )
}

// TODO: Counds 6s when executing a command using "sleep 3"
fn seconds_elapsed_since(date_time: DateTime<Utc>) -> i64 {
  Utc::now().timestamp() - date_time.timestamp()
}

pub fn execute_cmd(cmd: &str) {
  let start_time = Utc::now();

  // TODO: Rename things here.
  let mut c = Command::new("bash");

  let child = c
    .arg("-c")
    .arg(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

  // TODO: I think something in here is executing the process twice.
  //       (I think it's fixed now)

  let mut process = child.spawn().expect("Should execute command");
  let status = process.wait().expect("Should wait child");

  // TODO: This is too verbose.

  let elapsed_sec = seconds_elapsed_since(start_time);

  let stdout = process.stdout.unwrap();
  let stderr = process.stderr.unwrap();

  log_stdout(stdout, cmd, status, start_time, elapsed_sec);
  log_stderr(stderr, cmd, status, start_time, elapsed_sec);
}
