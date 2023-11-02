use std::sync::{Arc, Mutex};

use crate::server::{
  event_observe::make_event_observer,
  global_context::MainProcessState,
  global_context_installer::{install_state_from_file, InstallResult},
  http::start_http_server,
};

use super::{
  automata_manager::manage_automata, mouse_events::mouse_handler,
  results_command_exec::listen_results_execute_command,
};

use std::sync::mpsc;

fn install(config_path: &str, state: &mut MainProcessState) {
  println!("Config file path: {config_path}");

  let install_result = install_state_from_file(config_path, state);

  println!("{}", install_result.to_string());

  if let InstallResult::FileError(_) = install_result {
    std::process::exit(1);
  }
}

#[allow(clippy::too_many_lines)]
pub fn start(port: u32, config_path: &str) {
  let (sequence_sender, sequence_rec) = mpsc::channel();
  let (results_sender, results_rec) = mpsc::channel::<usize>();
  let sequence_sender_clone = sequence_sender.clone();

  let mut state = MainProcessState::new();

  install(config_path, &mut state);

  let main_process_state = Arc::new(Mutex::new(state));

  let (event_subscriber, mut event_notifier) = make_event_observer();

  std::thread::scope(|scope| {
    scope.spawn(|| listen_results_execute_command(results_rec, &main_process_state));
    scope.spawn(|| {
      manage_automata(
        &results_sender,
        sequence_rec,
        &mut event_notifier,
        &main_process_state,
      );
    });
    scope.spawn(|| mouse_handler(sequence_sender));
    scope.spawn(|| {
      start_http_server(
        port,
        config_path,
        sequence_sender_clone,
        Arc::new(Mutex::new(event_subscriber)),
        Arc::clone(&main_process_state),
      );
    });
  });
}
