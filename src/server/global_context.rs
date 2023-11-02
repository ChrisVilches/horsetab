use super::process_manager::ProcessManager;
use crate::{cmd::Cmd, sequence_automata::SequenceAutomata};

pub struct MainProcessState {
  pub commands: Vec<Cmd>,
  pub pre_script: String,
  pub automata: SequenceAutomata,
  pub process_manager: ProcessManager,
}

impl MainProcessState {
  pub fn new() -> Self {
    Self {
      commands: vec![],
      pre_script: String::new(),
      automata: SequenceAutomata::new(&[]),
      process_manager: ProcessManager::new(),
    }
  }
}
