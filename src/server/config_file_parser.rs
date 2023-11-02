use std::collections::BTreeSet;

use crate::{
  cmd::Cmd,
  sequence_automata::{AutomataInstruction, SequenceAutomata},
};

pub struct Configuration {
  pub commands: Vec<Cmd>,
  pub unreachable_sequences: Vec<String>,
  pub pre_script: String,
}

fn parse_lines(lines: &[String]) -> (Vec<Cmd>, String) {
  let mut commands = vec![];
  let mut other = vec![];

  for line in lines {
    Cmd::parse(line).map_or_else(
      |_| {
        other.push(line.clone());
      },
      |cmd| {
        commands.push(cmd);
      },
    );
  }

  (commands, other.join("\n"))
}

fn pluck_sequence(commands: &[Cmd]) -> Vec<&str> {
  commands
    .iter()
    .map(|c| c.sequence.as_ref())
    .collect::<Vec<&str>>()
}

fn sequence_is_reachable(sequence: &str, automata: &mut SequenceAutomata, id: usize) -> bool {
  let mut latest_result = None;
  for c in sequence.chars() {
    latest_result = automata.put(AutomataInstruction::Char(c));
  }

  latest_result.map_or(false, |res| res.contains(&id))
}

fn get_unreachable_sequences(sequences: &[&str]) -> Vec<String> {
  let mut automata = SequenceAutomata::new(sequences);

  let mut ids = (0..sequences.len()).collect::<BTreeSet<usize>>();

  for (i, seq) in sequences.iter().enumerate() {
    automata.put(AutomataInstruction::Reset);

    if sequence_is_reachable(seq, &mut automata, i) {
      ids.remove(&i);
    }
  }

  ids
    .iter()
    .map(|i| sequences[*i])
    .map(std::borrow::ToOwned::to_owned)
    .collect::<Vec<String>>()
}

impl Configuration {
  pub fn from_lines(lines: &[String]) -> Self {
    let (commands, pre_script) = parse_lines(lines);

    let sequences: Vec<&str> = pluck_sequence(&commands);
    let unreachable_sequences = get_unreachable_sequences(&sequences);

    Self {
      commands,
      unreachable_sequences,
      pre_script,
    }
  }

  pub fn get_sequences(&self) -> Vec<&str> {
    self
      .commands
      .iter()
      .map(|c| c.sequence.as_ref())
      .collect::<Vec<&str>>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test]
  fn test_get_unreachable_sequences_all_ok() {
    assert!(get_unreachable_sequences(&["abc", "bca"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "abb"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "aaa"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "abd"]).is_empty());
  }

  #[test]
  fn test_get_unreachable_sequences_some_fail() {
    assert_eq!(get_unreachable_sequences(&["abc", "abcc"]), vec!["abcc"]);
    assert_eq!(
      get_unreachable_sequences(&["abc", "abcc", "abccc"]),
      vec!["abcc", "abccc"]
    );
    assert_eq!(
      get_unreachable_sequences(&["abccc", "abcc", "abcx"]),
      vec!["abccc"]
    );
  }

  #[test]
  fn test_get_unreachable_sequences_same() {
    assert!(get_unreachable_sequences(&["abc", "abc"]).is_empty());
    assert!(get_unreachable_sequences(&["a", "a"]).is_empty());
  }

  fn string_vec<const N: usize>(strs: [&str; N]) -> Vec<String> {
    strs.iter().copied().map(|x| x.to_owned()).collect()
  }

  #[test]
  fn test_parse_lines_multiple_empty_lines() {
    let (cmds, other) = parse_lines(&string_vec([" ", "   ", " "]));
    assert!(cmds.is_empty());
    assert_eq!(other, " \n   \n ");
  }

  #[test]
  fn test_parse_lines_with_comments() {
    let (cmds, other) = parse_lines(&string_vec([" #", "  # .-.- aa ", "#", " #.-."]));
    assert!(cmds.is_empty());
    assert_eq!(other, " #\n  # .-.- aa \n#\n #.-.");
  }

  #[test]
  fn test_parse_lines_with_comments_2() {
    let (cmds, other) = parse_lines(&string_vec([".-.-  #"]));

    assert!(other.is_empty());
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0].sequence, ".-.-");
    assert_eq!(cmds[0].command, "#");
  }

  #[test_case("  .-.#a")]
  #[test_case("  .#b")]
  #[test_case(".#c")]
  #[test_case(" aaa  ")]
  #[test_case(" . source_something")]
  #[test_case("  something ")]
  fn test_parse_lines_with_non_morse_command(line: &str) {
    let (cmds, other) = parse_lines(&string_vec([line]));
    assert!(cmds.is_empty());
    assert!(!other.is_empty());
  }

  #[test_case([" .- x ", " ", " . xyz ", " .--  yyy"], &[".-", ".--"], &["x", "yyy"], " \n . xyz ")]
  #[test_case([" #.- x ", " . xyz ", " .--  yyy"], &[".--"], &["yyy"], " #.- x \n . xyz ")]
  #[test_case([" . abc ", " hello ", " # world", " .-.- cmd "], &[".-.-"], &["cmd"], " . abc \n hello \n # world")]
  fn test_parse_lines<const N: usize>(
    lines: [&str; N],
    expected_seq: &[&str],
    expected_cmd: &[&str],
    expected_other: &str,
  ) {
    let (cmds, other) = parse_lines(&string_vec(lines));

    let result_seq = cmds
      .iter()
      .map(|c| c.sequence.to_owned())
      .collect::<Vec<String>>();

    let result_cmd = cmds
      .iter()
      .map(|c| c.command.to_owned())
      .collect::<Vec<String>>();

    assert_eq!(result_seq, expected_seq);
    assert_eq!(result_cmd, expected_cmd);
    assert_eq!(other, expected_other);
  }
}
