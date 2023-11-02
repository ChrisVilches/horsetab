use super::config_file_parser::Configuration;
use super::global_context::MainProcessState;
use crate::sequence_automata::SequenceAutomata;
use crate::util::{ensure_shebang, read_lines_or_create};

pub enum InstallResult {
  Ok(usize),
  Unreachable((usize, Vec<String>)),
  FileError(std::io::Error),
}

impl ToString for InstallResult {
  fn to_string(&self) -> String {
    match self {
      Self::Ok(count) => format!("Installed {count} commands"),
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

fn assign_global_state(config: Configuration, state: &mut MainProcessState) {
  state.automata = SequenceAutomata::new(&config.get_sequences());
  state.commands = config.commands;
  state.pre_script = ensure_shebang(&config.pre_script);
}

pub fn install_state_from_file(config_path: &str, state: &mut MainProcessState) -> InstallResult {
  match read_lines_or_create(config_path) {
    Ok(lines) => {
      let config = Configuration::from_lines(&lines);

      let total = config.commands.len();
      let unreachable_sequences = config.unreachable_sequences.clone();

      assign_global_state(config, state);

      if unreachable_sequences.is_empty() {
        InstallResult::Ok(total)
      } else {
        InstallResult::Unreachable((total, unreachable_sequences))
      }
    }
    Err(err) => InstallResult::FileError(err),
  }
}
