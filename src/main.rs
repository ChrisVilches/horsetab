// TODO: I hope I can do things without using too much "clone"

use cmd_parser::Cmd;
use main_threads::{
  automata_manager::manage_automata, commands_installer::commands_install,
  mouse_events::listen_mouse_events, results_command_exec::listen_results_command_exec,
};
use std::sync::{atomic::AtomicBool, Mutex};

mod click_length_detector;
mod cmd;
mod cmd_parser;
mod constants;
mod logger;
mod main_threads;
mod sequence_automata;

fn main() {
  let (sequence_sender, sequence_rec) = crossbeam::channel::unbounded();
  let (results_sender, results_rec) = crossbeam::channel::unbounded::<usize>();

  let commands = Mutex::new(Vec::<Cmd>::new());

  let commands_changed = AtomicBool::new(false);

  crossbeam::scope(|scope| {
    scope.spawn(|_| listen_results_command_exec(&commands, results_rec));
    scope.spawn(|_| manage_automata(&commands, results_sender, sequence_rec, &commands_changed));
    scope.spawn(|_| listen_mouse_events(sequence_sender));
    scope.spawn(|_| commands_install(&commands, &commands_changed));
  })
  .expect("Should run all threads");
}
