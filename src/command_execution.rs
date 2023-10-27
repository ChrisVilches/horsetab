use crate::logger::{log_stderr, log_stdout};
use chrono::{DateTime, Local};
use std::process::{Command, Stdio};

fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

pub fn spawn_process(cmd: &str) {
  let start_time = Local::now();

  let mut child = Command::new("bash")
    .arg("-c")
    .arg(cmd)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Should execute command");

  let status = child.wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  let stdout = child.stdout.unwrap();
  let stderr = child.stderr.unwrap();

  log_stdout(stdout, cmd, status, start_time, elapsed_sec);
  log_stderr(stderr, cmd, status, start_time, elapsed_sec);
}
