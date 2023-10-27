use crate::sequence_automata::{AutomataInstruction, SequenceAutomata};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use super::event_observe::{EventObserver, EventType};

fn notify_event(event_observer: &Mutex<EventObserver>, instruction: AutomataInstruction) {
  event_observer
    .lock()
    .unwrap()
    .notify_with(|| match instruction {
      AutomataInstruction::Char(c) => EventType::SequenceItem(c),
      AutomataInstruction::Reset => EventType::SequenceReset,
    });
}

// TODO: Is all of this notify'ing expensive? Remember it happens on every click.
//       The first bits occur whether there are listeners or not. So maybe I could guard it even more.
//       Like add a "has_subscribers?" method, which sorta depends on the caller, but it's probably worth it.

fn notify_success(event_observer: &Mutex<EventObserver>) {
  event_observer
    .lock()
    .unwrap()
    .notify_with(|| EventType::FoundResults);
}

pub fn manage_automata(
  automata: &Mutex<SequenceAutomata>,
  results_sender: &Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
  event_observer: &Mutex<EventObserver>,
) {
  for instruction in sequence_rec {
    notify_event(event_observer, instruction);

    if let Some(results) = automata.lock().unwrap().put(instruction) {
      notify_success(event_observer);

      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
