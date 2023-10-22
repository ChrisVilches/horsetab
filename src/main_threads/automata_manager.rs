use std::sync::{
  atomic::{AtomicBool, Ordering},
  Mutex, MutexGuard,
};

use crossbeam::channel::{Receiver, Sender};

use crate::{
  cmd_parser::Cmd,
  sequence_automata::{AutomataInstruction, SequenceAutomata},
};

fn rebuild_automata(
  automata: &mut SequenceAutomata,
  commands_changed: &AtomicBool,
  commands: MutexGuard<Vec<Cmd>>,
) {
  if !commands_changed.load(Ordering::Relaxed) {
    return;
  }

  println!("Installing {} commands", commands.len());

  let sequences = commands
    .iter()
    .map(|c| c.sequence.as_ref())
    .collect::<Vec<&str>>();
  *automata = SequenceAutomata::new(&sequences);

  commands_changed.store(false, Ordering::Relaxed);
}

pub fn manage_automata(
  commands: &Mutex<Vec<Cmd>>,
  results_sender: Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
  commands_changed: &AtomicBool,
) {
  let mut automata = SequenceAutomata::new(&[""]);

  while let Ok(mouse_click_kind) = sequence_rec.recv() {
    rebuild_automata(
      &mut automata,
      commands_changed,
      commands.lock().expect("Should obtain lock"),
    );

    if let Some(results) = automata.put(mouse_click_kind) {
      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
