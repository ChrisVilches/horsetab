use crate::cmd::Cmd;
use crate::sequence_automata::{AutomataInstruction, SequenceAutomata};
use crate::util::clean_command_lines;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::{collections::BTreeSet, sync::Mutex};

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

fn parse_lines(lines: &[&str]) -> (Vec<Cmd>, Vec<String>) {
  let mut commands = vec![];
  let mut non_commands = vec![];

  for line in clean_command_lines(lines.iter().copied()) {
    Cmd::parse(&line).map_or_else(
      |_| {
        non_commands.push(line);
      },
      |cmd| {
        commands.push(cmd);
      },
    );
  }

  (commands, non_commands)
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

  let mut ids = sequences
    .iter()
    .enumerate()
    .map(|(i, _)| i)
    .collect::<BTreeSet<usize>>();

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

pub enum InstallResult {
  Ok(usize),
  // NoChange,
  Unreachable((usize, Vec<String>)),
  FileError(std::io::Error),
}

impl ToString for InstallResult {
  fn to_string(&self) -> String {
    match self {
      Self::Ok(count) => format!("Installed {count} commands"),
      // TODO: (NoChange) A bit harder to implement. The file contents have to be checked, not the Vec<Cmd>
      //       because the Vec<Cmd> is clean and ignores the commands that failed to parse, so sometimes
      //       "NoChange" would be returned simply because the Vec didn't change but maybe the commands that
      //       failed to be parsed did change.
      //       In simpler words: it's necessary to check if the file (string) changed, not the Vec<Cmd> result
      //       to avoid a wrong result.
      // Self::NoChange => "No modification made"
      Self::Unreachable((count, sequences)) => {
        let mut text = format!("Installed {count} commands, with some unreachable sequence(s):");

        for seq in sequences {
          text += "\n";
          text += seq;
        }

        text
      }
      Self::FileError(err) => format!("Cannot install commands from file: {err}"),
    }
  }
}

// TODO: In order to stabilize the config file grammar, I should write unit tests for this function.
// TODO: Write unit tests of the whole thing.
pub fn install_commands(
  config_path: &str,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
  pre_cmd: &Mutex<String>,
) -> InstallResult {
  match read_lines_or_create(config_path) {
    Ok(lines) => {
      let (cmds, non_cmds) = parse_lines(
        &lines
          .iter()
          .map(std::ops::Deref::deref)
          .collect::<Vec<&str>>(),
      );

      let sequences: Vec<&str> = pluck_sequence(&cmds);
      let unreachable_sequences = get_unreachable_sequences(&sequences);

      let total = cmds.len();
      *automata.lock().unwrap() = SequenceAutomata::new(&sequences);
      *commands.lock().unwrap() = cmds;
      *pre_cmd.lock().unwrap() = non_cmds.join("\n").trim().to_owned();

      if unreachable_sequences.is_empty() {
        InstallResult::Ok(total)
      } else {
        InstallResult::Unreachable((total, unreachable_sequences))
      }
    }
    Err(err) => InstallResult::FileError(err),
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

  #[test]
  fn test_parse_lines_multiple_empty_lines() {
    let (cmds, other) = parse_lines(&vec![" ", "   ", " "]);
    assert!(cmds.is_empty());
    assert!(other.is_empty());
  }

  #[test]
  fn test_parse_lines_with_comments() {
    let (cmds, other) = parse_lines(&vec![" #", "  # .-.- aa ", "#", " #.-."]);
    assert!(cmds.is_empty());
    assert!(other.is_empty());
  }

  #[test]
  fn test_parse_lines_with_comments_2() {
    let (cmds, other) = parse_lines(&vec![".-.-  #"]);

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
    let (cmds, other) = parse_lines(&vec![line]);
    assert!(cmds.is_empty());
    assert_eq!(other.len(), 1);
  }

  #[test_case(&[" .- x ", " . xyz ", " .--  yyy"], &[".-", ".--"], &["x", "yyy"], &[". xyz"])]
  #[test_case(&[" #.- x ", " . xyz ", " .--  yyy"], &[".--"], &["yyy"], &[". xyz"])]
  #[test_case(&[" . abc ", " hello ", " # world", " .-.- cmd "], &[".-.-"], &["cmd"], &[". abc", "hello"])]
  fn test_parse_lines(
    lines: &[&str],
    expected_seq: &[&str],
    expected_cmd: &[&str],
    expected_other: &[&str],
  ) {
    let (cmds, other) = parse_lines(lines);

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
