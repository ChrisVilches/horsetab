use chrono::{DateTime, Local};
use std::io::{stderr, stdout, BufReader, Write};
use std::io::{BufRead, Read};

use crate::util::format_date;

fn format_log_msg(msg: &str, logger_name: &str, pid: u32, date: DateTime<Local>) -> String {
  let date_fmt = format_date(date);
  format!("[{logger_name} {date_fmt} {pid:>6}] {msg}")
}

fn output_newline_if_missing<W>(msg: &str, out: &mut W)
where
  W: Write,
{
  if !msg.ends_with('\n') {
    out.write_all(b"\n").unwrap();
  }
}

pub fn log_stdout(pid: u32, msg: &str) {
  let mut out = stdout().lock();
  let msg_fmt = format_log_msg(msg, "stdout", pid, Local::now());
  out.write_all(msg_fmt.as_bytes()).unwrap();
  output_newline_if_missing(&msg_fmt, &mut out);
}

fn log_stderr(pid: u32, msg: &str) {
  let mut out = stderr().lock();
  let msg_fmt = format_log_msg(msg, "stderr", pid, Local::now());
  out.write_all(msg_fmt.as_bytes()).unwrap();
  output_newline_if_missing(&msg_fmt, &mut out);
}

pub fn redirect_output<R>(mut buf: BufReader<R>, pid: u32, stdout: bool)
where
  R: Read,
{
  let mut line = String::new();

  while buf.read_line(&mut line).unwrap() > 0 {
    if line.is_empty() {
      continue;
    }

    if stdout {
      log_stdout(pid, &line);
    } else {
      log_stderr(pid, &line);
    }

    line.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("aa", "stdout", 123, "[stdout 2023-01-02 00:11:22    123] aa")]
  #[test_case("aa", "stdout", 1, "[stdout 2023-01-02 00:11:22      1] aa")]
  #[test_case("aa", "stdout", 123123, "[stdout 2023-01-02 00:11:22 123123] aa")]
  #[test_case("aa", "stderr", 12312377, "[stderr 2023-01-02 00:11:22 12312377] aa")]
  #[test_case("", "stderr", 12312377, "[stderr 2023-01-02 00:11:22 12312377] ")]
  fn test_format_log_msg(msg: &str, logger_name: &str, pid: u32, expected: &str) {
    std::env::set_var("TZ", "Asia/Tokyo");

    let fixed_date = "2023-01-02 00:11:22 +0900";
    let date = DateTime::parse_from_str(fixed_date, "%Y-%m-%d %H:%M:%S %z")
      .unwrap()
      .with_timezone(&Local);

    let result = format_log_msg(msg, logger_name, pid, date);
    assert_eq!(result, expected);
  }
}
