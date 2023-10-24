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

// TODO: I think it's probably not that hard to parse comments.
//       A string like this would be valid:
//       .-.-.- #asdasd
//       Since it has a sequence but the command is simply a comment
/*
I just verified it (using this conf:  .-.- #asdasd):

[2023-10-25 04:48:26] #asdasd
Done in 0s
[2023-10-25 04:48:26] #asdasd
Done in 0s

So there are two cases (although I'd prefer to use one big generic regex)
it starts with a sequence (the rest can be a command or comment, whatever)
The first thing is the sequence

TODO: It's done, but it's a bit unstable since as soon as I modify the grammar of the command config file
      it'll break. So maybe implement some advanced parsing technique or whatever.
      Or maybe just one regex would do, but I'd like to test it more.
      For now leave it as is, and then remove this TODO if it doesn't seem too urgent.
*/

fn match_line(line: &str) -> Option<(&str, &str)> {
  let guard = REGEX.lock().unwrap();
  let re = guard.get_or_init(|| Regex::new(r"^\s*([.-]+)\s+(.+)$").unwrap());

  let mut capture = re.captures_iter(line).map(|c| c.extract());

  capture
    .next()
    .map(|(_, [sequence, command])| (sequence, command.trim()))
}

pub fn parse_command(line: &str) -> Result<Cmd> {
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
    assert_eq!(
      parse_command(" x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
    assert_eq!(
      parse_command(" .-.x-  x ").err().unwrap().to_string(),
      "Some commands have incorrect format"
    );
  }

  #[test]
  fn test_empty_line() {
    assert!(parse_command("").is_err());
    assert!(parse_command("  ").is_err());
  }

  #[test]
  fn test_command_parse() {
    let res = parse_command(" .-.-   some_cmd.sh    >>log ");
    assert!(res.is_ok());
    let cmd = res.unwrap();
    assert_eq!(cmd.command, "some_cmd.sh    >>log");
    assert_eq!(cmd.sequence, ".-.-");
  }

  #[test]
  fn test_command_parse_2() {
    let cmd = parse_command("  .-.- one .-.- two").unwrap();
    assert_eq!(cmd.sequence, ".-.-");
    assert_eq!(cmd.command, "one .-.- two")
  }
}
