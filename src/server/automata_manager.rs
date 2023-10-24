use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use crate::sequence_automata::{AutomataInstruction, SequenceAutomata};

pub fn manage_automata(
  automata: &Mutex<SequenceAutomata>,
  results_sender: &Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
) {
  for mouse_click_kind in sequence_rec {
    if let Some(results) = automata.lock().unwrap().put(mouse_click_kind) {
      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
