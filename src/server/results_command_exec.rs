use std::sync::Mutex;

use std::sync::mpsc::Receiver;

use crate::{cmd::spawn_process, cmd_parser::Cmd};

pub fn listen_results_execute_command(commands: &Mutex<Vec<Cmd>>, results_rec: Receiver<usize>) {
  for result_id in results_rec {
    let cmd = &commands.lock().expect("Should obtain lock")[result_id].command;
    spawn_process(cmd);
  }
}
