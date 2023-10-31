use crate::cmd::Cmd;
use crate::sequence_automata::SequenceAutomata;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

use super::config_file_parser::Configuration;

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

fn assign_global_state(
  config: Configuration,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
  shell_script: &Mutex<String>,
  interpreter: &Mutex<Vec<String>>,
) {
  *automata.lock().unwrap() = SequenceAutomata::new(&config.get_sequences());
  *commands.lock().unwrap() = config.commands;
  *shell_script.lock().unwrap() = config.shell_script;
  *interpreter.lock().unwrap() = config.interpreter;
}

pub fn install_commands(
  config_path: &str,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
  shell_script: &Mutex<String>,
  interpreter: &Mutex<Vec<String>>,
) -> InstallResult {
  match read_lines_or_create(config_path) {
    Ok(lines) => {
      let config = Configuration::from_lines(&lines);

      let total = config.commands.len();
      let unreachable_sequences = config.unreachable_sequences.clone();

      assign_global_state(config, automata, commands, shell_script, interpreter);

      if unreachable_sequences.is_empty() {
        InstallResult::Ok(total)
      } else {
        InstallResult::Unreachable((total, unreachable_sequences))
      }
    }
    Err(err) => InstallResult::FileError(err),
  }
}
