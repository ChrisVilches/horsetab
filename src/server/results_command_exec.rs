use std::sync::Mutex;

use std::sync::mpsc::Receiver;

use crate::{cmd::Cmd, command_execution::spawn_process};

pub fn listen_results_execute_command(
  pre_cmd: &Mutex<String>,
  commands: &Mutex<Vec<Cmd>>,
  results_rec: Receiver<usize>,
) {
  for result_id in results_rec {
    let cmd = &commands.lock().unwrap()[result_id].command;

    let process_result = spawn_process(&pre_cmd.lock().unwrap(), cmd);

    if let Err(e) = process_result {
      eprintln!("{e}");
    }
  }
}
