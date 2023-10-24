use std::sync::Mutex;

use std::sync::mpsc::Receiver;

use crate::{cmd::Cmd, command_execution::spawn_process};

pub fn listen_results_execute_command(commands: &Mutex<Vec<Cmd>>, results_rec: Receiver<usize>) {
  for result_id in results_rec {
    let cmd = &commands.lock().unwrap()[result_id].command;
    spawn_process(cmd);
  }
}
