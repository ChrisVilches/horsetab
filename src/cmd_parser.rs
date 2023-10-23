use std::{cell::OnceCell, sync::Mutex};

use anyhow::{bail, Result};
use regex::Regex;

pub struct Cmd {
  pub sequence: String,
  pub command: String,
}

impl Cmd {
  fn new(sequence: &str, command: &str) -> Self {
    Self {
      sequence: sequence.into(),
      command: command.into(),
    }
  }
}

static REGEX: Mutex<OnceCell<Regex>> = Mutex::new(OnceCell::new());

fn match_line(line: &str) -> Option<(&str, &str)> {
  let guard = REGEX.lock().unwrap();
  let re = guard.get_or_init(|| Regex::new(r"^\s*([.-]+)\s+(.+)$").unwrap());

  let mut capture = re.captures_iter(line).map(|c| c.extract());

  match capture.next() {
    Some((_, [sequence, command])) => Some((sequence, command.trim())),
    None => None,
  }
}

pub fn parse_cmd(line: &str) -> Result<Cmd> {
  match match_line(line) {
    Some((sequence, command)) => Ok(Cmd::new(sequence, command)),
    None => {
      bail!("Some commands have incorrect format")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_match_line() {
    assert_eq!(match_line("  .-.- x"), Some((".-.-", "x")));
    assert_eq!(match_line("  .  yx   "), Some((".", "yx")));
    assert_eq!(match_line("    yx   "), None);
    assert_eq!(match_line("    yx  ..--  "), None);
    assert_eq!(match_line(" .-.-x.- xxx "), None);
    assert_eq!(match_line(" .-.- x .-.- y"), Some((".-.-", "x .-.- y")));
  }

  #[test]
  fn test_error() {
    println!("{:?}", parse_cmd(" .-.x-  x ").is_err());
    assert_eq!(
      parse_cmd(" x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
    assert_eq!(
      parse_cmd(" .-.x-  x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
  }

  #[test]
  fn test_empty_line() {
    assert!(parse_cmd("").is_err());
    assert!(parse_cmd("  ").is_err());
  }

  #[test]
  fn test_command_parse() {
    let res = parse_cmd(" .-.-   some_cmd.sh    >>log ");
    assert!(res.is_ok());
    let cmd = res.unwrap();
    assert_eq!(cmd.command, "some_cmd.sh    >>log");
    assert_eq!(cmd.sequence, ".-.-");
  }

  #[test]
  fn test_command_parse_2() {
    let cmd = parse_cmd("  .-.- one .-.- two").unwrap();
    assert_eq!(cmd.sequence, ".-.-");
    assert_eq!(cmd.command, "one .-.- two")
  }
}
