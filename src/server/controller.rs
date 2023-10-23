use std::sync::{Arc, Mutex};

use crate::{
  cmd_parser::Cmd, sequence_automata::SequenceAutomata, server::commands_installer::InstallResult,
};

use super::{
  automata_manager::manage_automata, commands_installer::install_commands, http::start_http_server,
  mouse_events::mouse_handler, results_command_exec::listen_results_execute_command,
};

pub fn start(port: &str, config_path: &str) {
  let (sequence_sender, sequence_rec) = crossbeam::channel::unbounded();
  let (results_sender, results_rec) = crossbeam::channel::unbounded::<usize>();

  let commands = Arc::new(Mutex::new(Vec::<Cmd>::new()));

  let automata = Arc::new(Mutex::new(SequenceAutomata::new(&[""])));

  let install_result = install_commands(config_path, &automata, &commands);
  println!("{}", install_result.to_string());

  if let InstallResult::Error(_) = install_result {
    std::process::exit(1);
  }

  crossbeam::scope(|scope| {
    scope.spawn(|_| listen_results_execute_command(&commands, &results_rec));
    scope.spawn(|_| manage_automata(&automata, &results_sender, &sequence_rec));
    scope.spawn(|_| mouse_handler(sequence_sender));
    scope.spawn(|_| {
      println!("Listening on port {port}");
      start_http_server(
        port,
        config_path,
        Arc::clone(&automata),
        Arc::clone(&commands),
      );
    });
  })
  .expect("Should run all threads");
}
