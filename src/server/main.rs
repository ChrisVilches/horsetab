use std::sync::{Arc, Mutex};

use crate::{
  cmd::Cmd,
  sequence_automata::SequenceAutomata,
  server::{
    commands_installer::InstallResult, event_observe::EventObserver, http::start_http_server,
  },
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

  // TODO: Can this be done without Arc????
  let event_observer = Arc::new(Mutex::new(EventObserver::new()));

  println!("Config file path: {config_path}");

  let install_result = install_commands(config_path, &automata, &commands);

  println!("{}", install_result.to_string());

  match install_result {
    InstallResult::SyntaxError(_) => println!("You must edit commands to retry installing them"),
    InstallResult::FileError(_) => std::process::exit(1),
    _ => {}
  }

  std::thread::scope(|scope| {
    scope.spawn(|| listen_results_execute_command(&commands, results_rec));
    scope.spawn(|| manage_automata(&automata, &results_sender, sequence_rec, &event_observer));
    scope.spawn(|| mouse_handler(sequence_sender));
    scope.spawn(|| {
      start_http_server(
        port,
        config_path,
        Arc::clone(&event_observer),
        Arc::clone(&automata),
        Arc::clone(&commands),
      );
    });
  });
}
