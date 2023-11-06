use anyhow::Result;
use bincode::ErrorKind;
use chrono::{
  format::{DelayedFormat, StrftimeItems},
  DateTime, Local,
};
use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::{fs::OpenOptions, ops::Deref};

pub fn format_date<'a>(date: DateTime<Local>) -> DelayedFormat<StrftimeItems<'a>> {
  date.format("%Y-%m-%d %H:%M:%S")
}

pub fn seconds_elapsed(lo: DateTime<Local>, hi: Option<DateTime<Local>>) -> i64 {
  hi.unwrap_or_else(Local::now).timestamp() - lo.timestamp()
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

pub struct PayloadOverwriter {
  inner: Vec<u8>,
}

impl PayloadOverwriter {
  pub const fn new() -> Self {
    Self { inner: vec![] }
  }

  pub fn overwrite_serialize<T: Serialize>(&mut self, value: T) -> Result<(), Box<ErrorKind>> {
    assert!(self.inner.len() < 100);
    self.inner.clear();
    bincode::serialize_into(&mut self.inner, &value)
  }
}

impl Deref for PayloadOverwriter {
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    self.inner.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use crate::ipc_tcp::EventType;

  use super::*;

  #[test]
  fn test_payload_overwriter() {
    let mut payload = PayloadOverwriter::new();
    payload
      .overwrite_serialize(EventType::SequenceReset)
      .unwrap();
    assert_eq!(payload.len(), 4);
    payload
      .overwrite_serialize(EventType::SequenceItem('a'))
      .unwrap();
    assert_eq!(payload.len(), 5);
    payload
      .overwrite_serialize(EventType::FoundResults)
      .unwrap();
    assert_eq!(payload.len(), 4);
  }
}
