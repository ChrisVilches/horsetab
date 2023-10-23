use std::sync::Mutex;

use crossbeam::channel::Receiver;

use crate::{cmd::spawn_process, cmd_parser::Cmd};

pub fn listen_results_execute_command(commands: &Mutex<Vec<Cmd>>, results_rec: &Receiver<usize>) {
  while let Ok(result_id) = results_rec.recv() {
    let cmd = &commands.lock().expect("Should obtain lock")[result_id].command;
    spawn_process(cmd);
  }
}
