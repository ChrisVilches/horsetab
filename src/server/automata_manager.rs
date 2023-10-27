use crate::sequence_automata::{AutomataInstruction, SequenceAutomata};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use super::event_observe::{EventNotifier, EventType};

fn notify_event(event_notifier: &mut EventNotifier, instruction: AutomataInstruction) {
  event_notifier.notify_with(|| match instruction {
    AutomataInstruction::Char(c) => EventType::SequenceItem(c),
    AutomataInstruction::Reset => EventType::SequenceReset,
  });
}

// TODO: Is all of this notify'ing expensive? Remember it happens on every click.
//       The first bits occur whether there are listeners or not. So maybe I could guard it even more.
//       Like add a "has_subscribers?" method, which sorta depends on the caller, but it's probably worth it.

fn notify_success(event_notifier: &mut EventNotifier) {
  event_notifier.notify_with(|| EventType::FoundResults);
}

pub fn manage_automata(
  automata: &Mutex<SequenceAutomata>,
  results_sender: &Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
  event_notifier: &mut EventNotifier,
) {
  for instruction in sequence_rec {
    notify_event(event_notifier, instruction);

    if let Some(results) = automata.lock().unwrap().put(instruction) {
      notify_success(event_notifier);

      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
