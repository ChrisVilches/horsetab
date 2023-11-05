use std::sync::{Arc, Mutex};

use super::process_manager::ProcessManager;
use crate::{cmd::Cmd, sequence_automata::SequenceAutomata};

pub struct MainProcessState {
  pub commands: Vec<Cmd>,
  pub pre_script: String,
  pub automata: SequenceAutomata,
  pub process_manager: Arc<Mutex<ProcessManager>>,
  pub interpreter: String,
}

impl MainProcessState {
  pub fn new(interpreter: &str) -> Self {
    let process_manager = Arc::new(Mutex::new(ProcessManager::new()));

    ProcessManager::start_garbage_collection(
      Arc::clone(&process_manager),
      std::time::Duration::from_secs(2),
    );

    Self {
      commands: vec![],
      pre_script: String::new(),
      automata: SequenceAutomata::new(&[]),
      process_manager,
      interpreter: interpreter.to_owned(),
    }
  }
}
