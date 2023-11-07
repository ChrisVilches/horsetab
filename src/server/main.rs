use std::{
  collections::HashMap,
  net::{TcpListener, TcpStream},
  sync::{Arc, Mutex},
};

use crate::{
  event_observe::{notify_watch_observers, EventType},
  ipc_tcp::start_tcp_server,
  server::{
    global_context::MainProcessState,
    global_context_installer::{install_state_from_file, InstallResult},
    http::start_http_server,
  },
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
pub fn start(port: u16, config_path: &str, interpreter: &str) {
  let (sequence_sender, sequence_rec) = mpsc::channel();
  let (results_sender, results_rec) = mpsc::channel::<usize>();
  let sequence_sender_clone = sequence_sender.clone();

  let mut state = MainProcessState::new(interpreter);

  install(config_path, &mut state);

  let main_process_state = Arc::new(Mutex::new(state));

  let (events_sender, events_rec) = mpsc::channel::<EventType>();

  let observers: Mutex<HashMap<u16, TcpStream>> = Mutex::new(HashMap::new());

  let tcp_listener = TcpListener::bind("0.0.0.0:0").unwrap();

  std::thread::scope(|scope| {
    scope.spawn(|| listen_results_execute_command(results_rec, &main_process_state));
    scope.spawn(|| notify_watch_observers(events_rec.into_iter(), &observers));
    scope.spawn(|| start_tcp_server(&tcp_listener, &observers));

    scope.spawn(|| {
      manage_automata(
        &results_sender,
        sequence_rec,
        &events_sender,
        &main_process_state,
      );
    });
    scope.spawn(|| mouse_handler(sequence_sender));
    scope.spawn(|| {
      start_http_server(
        port,
        tcp_listener.local_addr().unwrap().port(),
        config_path,
        sequence_sender_clone,
        Arc::clone(&main_process_state),
      );
    });
  });
}
