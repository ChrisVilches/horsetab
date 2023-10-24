use std::sync::{Arc, Mutex};

use crate::{
  cmd_parser::Cmd,
  sequence_automata::SequenceAutomata,
  server::{commands_installer::InstallResult, http::start_http_server},
};

use super::{
  automata_manager::manage_automata, commands_installer::install_commands,
  mouse_events::mouse_handler, results_command_exec::listen_results_execute_command,
};

use std::sync::mpsc;

pub fn start(port: u32, config_path: &str) {
  let (sequence_sender, sequence_rec) = mpsc::channel();
  let (results_sender, results_rec) = mpsc::channel::<usize>();

  let commands = Arc::new(Mutex::new(Vec::<Cmd>::new()));

  let automata = Arc::new(Mutex::new(SequenceAutomata::new(&[""])));

  let install_result = install_commands(config_path, &automata, &commands);
  println!("{}", install_result.to_string());

  if let InstallResult::Error(_) = install_result {
    println!("You must edit commands to retry installing them");
  }

  std::thread::scope(|scope| {
    scope.spawn(|| listen_results_execute_command(&commands, results_rec));
    scope.spawn(|| manage_automata(&automata, &results_sender, sequence_rec));
    scope.spawn(|| mouse_handler(sequence_sender));
    scope.spawn(|| {
      start_http_server(
        port,
        config_path,
        Arc::clone(&automata),
        Arc::clone(&commands),
      );
    });
  });
}
