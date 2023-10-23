use crate::{
  cmd::read_commands,
  cmd_parser::Cmd,
  sequence_automata::{AutomataInstruction, SequenceAutomata},
};
use std::{collections::BTreeSet, sync::Mutex};

fn get_sequences(commands: &Vec<Cmd>) -> Vec<&str> {
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

  if let Some(res) = latest_result {
    res.contains(&id)
  } else {
    false
  }
}

fn get_unreachable_sequences(sequences: &[&str]) -> Vec<usize> {
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

  ids.iter().cloned().collect()
}

fn format_binary_to_morse(s: &str) -> String {
  s.chars()
    .map(|c| if c == '0' { '.' } else { '-' })
    .collect()
}

pub enum InstallResult {
  Ok(usize),
  Unreachable((usize, Vec<String>)),
  Error(anyhow::Error),
}

impl ToString for InstallResult {
  fn to_string(&self) -> String {
    match self {
      InstallResult::Ok(count) => format!("Installed {count} commands"),
      InstallResult::Unreachable((count, sequences)) => {
        let mut text = format!("Installed {count} commands, with some unreachable sequence(s):");

        for seq in sequences {
          text += "\n";
          text += seq;
        }

        text
      }
      InstallResult::Error(err) => format!("Failed to install commands: {err}").to_owned(),
    }
  }
}

pub fn install_commands(
  config_path: &str,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
) -> InstallResult {
  let mut commands_guard = commands.lock().expect("Should obtain lock");

  match read_commands(&config_path) {
    Ok(cmds) => *commands_guard = cmds,
    Err(err) => return InstallResult::Error(err),
  }

  let sequences = get_sequences(&commands_guard);

  let unreachable_sequences = get_unreachable_sequences(&sequences)
    .iter()
    .map(|i| sequences[*i])
    .collect::<Vec<&str>>();

  *automata.lock().unwrap() = SequenceAutomata::new(&sequences);

  if !unreachable_sequences.is_empty() {
    return InstallResult::Unreachable((
      sequences.len(),
      unreachable_sequences
        .into_iter()
        .map(format_binary_to_morse)
        .collect::<Vec<String>>(),
    ));
  }

  InstallResult::Ok(sequences.len())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_unreachable_sequences_all_ok() {
    assert!(get_unreachable_sequences(&["abc", "bca"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "abb"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "aaa"]).is_empty());
    assert!(get_unreachable_sequences(&["abc", "abd"]).is_empty());
  }

  #[test]
  fn test_get_unreachable_sequences_some_fail() {
    assert_eq!(get_unreachable_sequences(&["abc", "abcc"]), vec![1]);
    assert_eq!(
      get_unreachable_sequences(&["abc", "abcc", "abccc"]),
      vec![1, 2]
    );
    assert_eq!(
      get_unreachable_sequences(&["abccc", "abcc", "abcx"]),
      vec![0]
    );
  }

  #[test]
  fn test_get_unreachable_sequences_same() {
    assert!(get_unreachable_sequences(&["abc", "abc"]).is_empty());
    assert!(get_unreachable_sequences(&["a", "a"]).is_empty());
  }
}
