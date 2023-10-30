use std::{cell::OnceCell, sync::Mutex};

use anyhow::{bail, Result};
use regex::Regex;

pub struct Cmd {
  pub sequence: String,
  pub command: String,
}

static REGEX: Mutex<OnceCell<Regex>> = Mutex::new(OnceCell::new());

fn match_line(line: &str) -> Option<(&str, &str)> {
  let guard = REGEX.lock().unwrap();
  let re = guard.get_or_init(|| Regex::new(r"^\s*([.-]{2,})\s+(.+)$").unwrap());

  let mut capture = re.captures_iter(line).map(|c| c.extract());

  capture
    .next()
    .map(|(_, [sequence, command])| (sequence, command.trim()))
}

impl Cmd {
  pub fn parse(line: &str) -> Result<Self> {
    match match_line(line) {
      Some((sequence, command)) => Ok(Self {
        sequence: sequence.into(),
        command: command.into(),
      }),
      None => {
        bail!("Some commands have incorrect format")
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_match_line() {
    assert_eq!(match_line("  .-.- x"), Some((".-.-", "x")));
    assert_eq!(match_line("  ..  yx   "), Some(("..", "yx")));
    assert_eq!(match_line("    yx   "), None);
    assert_eq!(match_line("    yx  ..--  "), None);
    assert_eq!(match_line(" .-.-x.- xxx "), None);
    assert_eq!(match_line(" .-.- x .-.- y"), Some((".-.-", "x .-.- y")));
  }

  #[test]
  fn test_error() {
    assert_eq!(
      Cmd::parse(" x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
    assert_eq!(
      Cmd::parse(" .-.x-  x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
    assert_eq!(
      Cmd::parse(" . x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
  }

  #[test]
  fn test_empty_line() {
    assert!(Cmd::parse("").is_err());
    assert!(Cmd::parse("  ").is_err());
  }

  #[test]
  fn test_command_parse() {
    let res = Cmd::parse(" .-.-   some_cmd.sh    >>log ");
    assert!(res.is_ok());
    let cmd = res.unwrap();
    assert_eq!(cmd.command, "some_cmd.sh    >>log");
    assert_eq!(cmd.sequence, ".-.-");
  }

  #[test]
  fn test_command_parse_2() {
    let cmd = Cmd::parse("  .-.- one .-.- two").unwrap();
    assert_eq!(cmd.sequence, ".-.-");
    assert_eq!(cmd.command, "one .-.- two")
  }
}
