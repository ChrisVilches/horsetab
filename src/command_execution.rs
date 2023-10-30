use crate::{
  constants::DEFAULT_INTERPRETER,
  logger::{log_stderr, log_stdout},
  util::parse_shebang_or_default,
};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::process::{Command, Stdio};

fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

pub fn spawn_process(pre_cmd: &str, cmd: &str) -> Result<()> {
  let start_time = Local::now();

  let full_command = format!("{pre_cmd}\n{cmd}");

  let interpreter = parse_shebang_or_default(pre_cmd, &DEFAULT_INTERPRETER);

  let mut child = Command::new(&interpreter[0])
    .args(&interpreter[1..])
    .arg(&full_command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .with_context(|| {
      format!("Cannot execute command using {interpreter:?}.\nCommand(s) executed: {full_command}")
    })?;

  let status = child.wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  let stdout = child.stdout.unwrap();
  let stderr = child.stderr.unwrap();

  log_stdout(stdout, cmd, status, start_time, elapsed_sec);
  log_stderr(stderr, cmd, status, start_time, elapsed_sec);

  Ok(())
}
