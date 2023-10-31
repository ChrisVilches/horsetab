use crate::logger::{log_stderr, log_stdout};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::process::{Child, Command, Stdio};

fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

fn create_child(interpreter: &Vec<String>, shell_script: &str, cmd: &str) -> Result<Child> {
  let full_command = format!("{shell_script}\n{cmd}");

  Command::new(&interpreter[0])
    .args(&interpreter[1..])
    .arg(&full_command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .with_context(|| {
      format!("Cannot execute command using {interpreter:?}.\nCommand(s) executed: {full_command}")
    })
}

pub fn spawn_process(interpreter: &Vec<String>, shell_script: &str, cmd: &str) -> Result<()> {
  let start_time = Local::now();

  let mut child = create_child(interpreter, shell_script, cmd)?;

  let status = child.wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  log_stdout(child.stdout.unwrap(), cmd, status, start_time, elapsed_sec);
  log_stderr(child.stderr.unwrap(), cmd, status, start_time, elapsed_sec);

  Ok(())
}
