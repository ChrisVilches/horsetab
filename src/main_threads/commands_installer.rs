use crate::{cmd::read_commands, cmd_parser::Cmd, sequence_automata::SequenceAutomata};
use std::{env, sync::Mutex};

fn get_sequences(commands: &Vec<Cmd>) -> Vec<&str> {
  commands
    .iter()
    .map(|c| c.sequence.as_ref())
    .collect::<Vec<&str>>()
}

pub fn commands_install(automata: &Mutex<SequenceAutomata>, commands: &Mutex<Vec<Cmd>>) {
  let args: Vec<String> = env::args().collect();
  let config_file_path = &args[1];

  let mut commands_guard = commands.lock().expect("Should obtain lock");
  *commands_guard = read_commands(config_file_path).expect("Should be able to read commands");

  let sequences = get_sequences(&commands_guard);

  *automata.lock().unwrap() = SequenceAutomata::new(&sequences);
}
