use anyhow::{bail, Result};

fn parse_sequence(seq: &str) -> Result<String> {
  let mut result = String::new();

  for c in seq.chars() {
    result.push(if c == '.' {
      '0'
    } else if c == '-' {
      '1'
    } else {
      bail!("Sequence is incorrect")
    });
  }

  Ok(result)
}

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

pub fn parse_cmd(text_line: &str) -> Result<Option<Cmd>> {
  let line = text_line.trim();

  if line.is_empty() {
    return Ok(None);
  }

  match line.find(' ') {
    Some(first_space) => {
      let (seq, cmd) = line.split_at(first_space);

      Ok(Some(Cmd::new(&parse_sequence(seq)?, cmd.trim())))
    }
    None => {
      bail!("Line should contain at least one space")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error() {
    assert_eq!(
      parse_cmd(" x ").err().unwrap().to_string(),
      "Line should contain at least one space"
    );
    assert_eq!(
      parse_cmd(" .-.x-  x ").err().unwrap().to_string(),
      "Sequence is incorrect"
    );
  }

  #[test]
  fn test_empty_line() {
    assert!(parse_cmd("").is_ok());
    assert!(parse_cmd("  ").is_ok());
    assert!(parse_cmd("  ").unwrap().is_none());
  }

  #[test]
  fn test_command_parse() {
    let res = parse_cmd(" .-.-   some_cmd.sh    >>log ");
    assert!(res.is_ok());
    let opt = res.unwrap();
    assert!(opt.is_some());
    let cmd = opt.unwrap();
    assert_eq!(cmd.command, "some_cmd.sh    >>log");
    assert_eq!(cmd.sequence, "0101");
  }
}
