use anyhow::Result;
use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Local,
};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

pub fn effectful_format_bytes_merge_newlines(string: &mut [u8], n: usize, prev_char: u8) -> &[u8] {
  let mut j = 0;
  let mut last_was_newline = false;
  for i in 0..n {
    if string[i] != b'\n' || !last_was_newline {
      string[j] = string[i];
      j += 1;
    }

    last_was_newline = string[i] == b'\n';
  }
  let leading_extra_newline = n > 0 && prev_char == string[0] && prev_char == b'\n';
  let start = usize::from(leading_extra_newline);

  &string[start..j]
}

pub fn format_date<'a>(date: DateTime<Local>) -> DelayedFormat<StrftimeItems<'a>> {
  date.format("%Y-%m-%d %H:%M:%S")
}

pub fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

pub fn read_lines_or_create(file_path: &str) -> Result<Vec<String>, std::io::Error> {
  let file = OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(file_path)?;

  let reader = BufReader::new(file);

  reader
    .lines()
    .collect::<Result<Vec<String>, std::io::Error>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("hello\nworldxxx", 11, b'a', "hello\nworld")]
  #[test_case("\nhello\n\nworld", 13, b'\n', "hello\nworld")]
  #[test_case("\n\n", 0, b'\n', "")]
  #[test_case("\n", 1, b'\n', "")]
  #[test_case("\n\n", 1, b'\n', "")]
  #[test_case("\n\n", 1, b'f', "\n")]
  #[test_case("\n\n", 2, b'\n', "")]
  #[test_case("\n\n", 2, b'x', "\n")]
  #[test_case("hello\n\n", 6, b'f', "hello\n")]
  #[test_case("hello\n\n", 7, b'f', "hello\n")]
  #[test_case("hello\n\nx", 8, b'\n', "hello\nx")]
  fn test_effectful_format_merge_newlines(input: &str, len: usize, prev_char: u8, expected: &str) {
    let mut bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
    let slice = bytes.as_mut_slice();

    let result = effectful_format_bytes_merge_newlines(slice, len, prev_char);
    assert_eq!(String::from_utf8_lossy(result), expected);
  }
}
