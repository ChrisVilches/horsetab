use std::sync::Mutex;

use crossbeam::channel::Receiver;

use crate::{cmd::execute_cmd, cmd_parser::Cmd};

pub fn listen_results_command_exec(commands: &Mutex<Vec<Cmd>>, results_rec: Receiver<usize>) {
  while let Ok(result_id) = results_rec.recv() {
    let guard = commands.lock().expect("Should obtain lock");
    let cmd = &guard[result_id].command;
    execute_cmd(cmd);
  }
}
