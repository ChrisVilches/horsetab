use anyhow::Result;
use std::io::Write;
use std::process::ExitStatus;
use std::{
  io::{self, Read},
  process::{ChildStderr, ChildStdout},
};

use chrono::{DateTime, Local};

fn get_failure_status_code(status: ExitStatus) -> Option<i32> {
  if status.success() {
    return None;
  }

  status.code()
}

fn build_log_header(cmd: &str, date_time: DateTime<Local>) -> String {
  let date_fmt = date_time.format("%Y-%m-%d %H:%M:%S");
  format!("[{date_fmt}] {cmd}")
}

fn build_log_footer(elapsed_sec: i64, status: ExitStatus) -> String {
  let done = format!("Done in {elapsed_sec}s");
  match get_failure_status_code(status) {
    Some(code) => format!("{done} - Exit code {code}"),
    None => done,
  }
}

fn log<W, R>(
  mut writer: W,
  mut content: R,
  cmd: &str,
  status: ExitStatus,
  start_time: DateTime<Local>,
  elapsed_sec: i64,
) -> Result<()>
where
  R: Read,
  W: Write,
{
  let header = build_log_header(cmd, start_time);
  let footer = build_log_footer(elapsed_sec, status);

  writeln!(writer, "{header}")?;
  io::copy(&mut content, &mut writer)?;
  writeln!(writer, "{footer}")?;
  Ok(())
}

pub fn log_stderr(
  content: ChildStderr,
  cmd: &str,
  status: ExitStatus,
  start_time: DateTime<Local>,
  elapsed_sec: i64,
) {
  let writer = io::stderr().lock();
  log(writer, content, cmd, status, start_time, elapsed_sec).unwrap();
}

pub fn log_stdout(
  content: ChildStdout,
  cmd: &str,
  status: ExitStatus,
  start_time: DateTime<Local>,
  elapsed_sec: i64,
) {
  let writer = io::stdout().lock();
  log(writer, content, cmd, status, start_time, elapsed_sec).unwrap();
}
