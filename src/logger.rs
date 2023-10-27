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

fn reader_to_string<R>(mut reader: R) -> Result<String>
where
  R: Read,
{
  let mut res = String::new();
  reader.read_to_string(&mut res)?;
  Ok(res.trim().to_owned())
}

fn log<W, R>(
  mut writer: W,
  content: R,
  cmd: &str,
  status: ExitStatus,
  start_time: DateTime<Local>,
  elapsed_sec: i64,
  skip_if_empty: bool,
) -> Result<()>
where
  R: Read,
  W: Write,
{
  let header = build_log_header(cmd, start_time);
  let footer = build_log_footer(elapsed_sec, status);
  let content = reader_to_string(content)?;

  // TODO: Not sure if this logic works for every case. Try to refactor it
  //       to make it more easily understandable and unit tests each bit (functions, etc).
  //       Also it's easier to test if I redirect the outputs and then open the files on two horizontal individual terminals.
  //       DONE, just test more.
  let skip = skip_if_empty && content.is_empty();

  if !skip {
    for text in [header, content, footer]
      .into_iter()
      .map(|line| line.trim().to_owned())
      .filter(|text| !text.is_empty())
    {
      writeln!(writer, "{text}")?;
    }
  }

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
  log(writer, content, cmd, status, start_time, elapsed_sec, true).unwrap();
}

// TODO: But what if I just use a Rust logger crate? lol

pub fn log_stdout(
  content: ChildStdout,
  cmd: &str,
  status: ExitStatus,
  start_time: DateTime<Local>,
  elapsed_sec: i64,
) {
  let writer = io::stdout().lock();
  log(writer, content, cmd, status, start_time, elapsed_sec, false).unwrap();
}
