use crate::{cmd::read_commands, cmd_parser::Cmd};
use std::{
  env,
  sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
  },
};

pub fn commands_install(commands: &Mutex<Vec<Cmd>>, commands_changed: &AtomicBool) {
  let args: Vec<String> = env::args().collect();
  let config_file_path = &args[1];

  let mut guard = commands.lock().expect("Should obtain lock");
  *guard = read_commands(config_file_path).expect("Should be able to read commands");

  commands_changed.store(true, Ordering::Relaxed);
}
