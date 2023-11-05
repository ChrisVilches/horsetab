use super::global_context::MainProcessState;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;

pub fn listen_results_execute_command(
  results_rec: Receiver<usize>,
  state: &Mutex<MainProcessState>,
) {
  for result_id in results_rec {
    let state_guard = state.lock().unwrap();

    let cmd = state_guard.commands[result_id].command.clone();

    let start_result = state_guard.process_manager.lock().unwrap().start(
      &state_guard.interpreter,
      &state_guard.pre_script,
      &cmd,
    );

    if let Err(e) = start_result {
      eprintln!("{e}");
    }
  }
}
