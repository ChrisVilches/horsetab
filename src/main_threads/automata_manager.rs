use std::sync::Mutex;

use crossbeam::channel::{Receiver, Sender};

use crate::sequence_automata::{AutomataInstruction, SequenceAutomata};

pub fn manage_automata(
  aut: &Mutex<SequenceAutomata>,
  results_sender: &Sender<usize>,
  sequence_rec: &Receiver<AutomataInstruction>,
) {
  while let Ok(mouse_click_kind) = sequence_rec.recv() {
    if let Some(results) = aut.lock().unwrap().put(mouse_click_kind) {
      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
