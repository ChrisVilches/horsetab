use std::sync::{
  atomic::{AtomicBool, Ordering},
  Mutex, MutexGuard,
};

use crossbeam::channel::{Receiver, Sender};

use crate::{cmd_parser::Cmd, constants::MouseClickKind, sequence_automata::SequenceAutomata};

fn rebuild_automata(
  automata: &mut SequenceAutomata,
  commands_changed: &AtomicBool,
  commands: MutexGuard<Vec<Cmd>>,
) {
  if !commands_changed.load(Ordering::Relaxed) {
    return;
  }

  println!("Installing {} commands", commands.len());

  for (i, cmd) in commands.iter().enumerate() {
    automata.add_sequence(cmd.sequence.clone(), i);
  }

  commands_changed.store(false, Ordering::Relaxed);
}

pub fn manage_automata(
  commands: &Mutex<Vec<Cmd>>,
  results_sender: Sender<usize>,
  sequence_rec: Receiver<MouseClickKind>,
  commands_changed: &AtomicBool,
) {
  let mut automata = SequenceAutomata::new();

  while let Ok(mouse_click_kind) = sequence_rec.recv() {
    rebuild_automata(
      &mut automata,
      &commands_changed,
      commands.lock().expect("Should obtain lock"),
    );

    if let Some(results) = automata.execute_input(mouse_click_kind) {
      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
