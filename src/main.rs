#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_unrelated)]

use cmd_parser::Cmd;
use main_threads::{
  automata_manager::manage_automata, commands_installer::commands_install,
  mouse_events::mouse_handler, results_command_exec::listen_results_execute_command,
};
use std::sync::{atomic::AtomicBool, Mutex};

mod click_sequence_detector;
mod cmd;
mod cmd_parser;
mod logger;
mod main_threads;
mod sequence_automata;

fn main() {
  let (sequence_sender, sequence_rec) = crossbeam::channel::unbounded();
  let (results_sender, results_rec) = crossbeam::channel::unbounded::<usize>();

  let commands = Mutex::new(Vec::<Cmd>::new());

  let commands_changed = AtomicBool::new(false);

  crossbeam::scope(|scope| {
    scope.spawn(|_| listen_results_execute_command(&commands, &results_rec));
    scope.spawn(|_| manage_automata(&commands, &results_sender, &sequence_rec, &commands_changed));
    scope.spawn(|_| mouse_handler(sequence_sender));
    scope.spawn(|_| commands_install(&commands, &commands_changed));
  })
  .expect("Should run all threads");
}
