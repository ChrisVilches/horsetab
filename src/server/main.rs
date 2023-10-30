use std::sync::{Arc, Mutex};

use crate::{
  cmd::Cmd,
  sequence_automata::SequenceAutomata,
  server::{
    commands_installer::InstallResult, event_observe::make_event_observer, http::start_http_server,
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

  let pre_cmd = Arc::new(Mutex::new(String::new()));

  let automata = Arc::new(Mutex::new(SequenceAutomata::new(&[""])));

  let (event_subscriber, mut event_notifier) = make_event_observer();

  println!("Config file path: {config_path}");

  let install_result = install_commands(config_path, &automata, &commands, &pre_cmd);

  println!("{}", install_result.to_string());

  if let InstallResult::FileError(_) = install_result {
    std::process::exit(1);
  }

  let sequence_sender_clone = sequence_sender.clone();

  // TODO: Not 100% related to env variables, but it's related to https://github.com/ChrisVilches/horsetab/issues/7
  //       I think I should create a method that reads a file into a struct that contains the following things:
  //       (enum) Ok{ commands, env_variables, unreachable } or Error(Kind)
  //       It's similar to what I already have, but it should parse an entire file and return that struct.
  //       The warning of "unreachable" should be displayed, similar to what I have.
  //       If there's at least ONE error, it should NOT install the commands, and should print the error.
  //       - On `serve`, it should start with the error message, and don't install anything
  //       - The `watch` command should show only the commands that are already installed, without parsing the file
  //         again. In other words, don't read the file again, simply query the commands data structure that
  //         already exists.
  //
  //       Related to envs... I'm gonna try the approach in which I remove all the ..-.-.- command lines, and
  //       leave a file with only the non-command stuff, then execute that one along with each command.
  //       That reuses the power of sh/bash/dash/zsh/etc.

  std::thread::scope(|scope| {
    scope.spawn(|| listen_results_execute_command(&pre_cmd, &commands, results_rec));
    scope.spawn(|| {
      manage_automata(
        &automata,
        &results_sender,
        sequence_rec,
        &mut event_notifier,
      );
    });
    scope.spawn(|| mouse_handler(sequence_sender));
    scope.spawn(|| {
      start_http_server(
        port,
        config_path,
        sequence_sender_clone,
        Arc::new(Mutex::new(event_subscriber)),
        Arc::clone(&automata),
        Arc::clone(&commands),
        Arc::clone(&pre_cmd),
      );
    });
  });
}
