use super::event_observe::{EventNotifier, EventType};
use super::global_context::MainProcessState;
use crate::sequence_automata::AutomataInstruction;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

fn notify_event(event_notifier: &mut EventNotifier, instruction: AutomataInstruction) {
  event_notifier.notify_with(|| match instruction {
    AutomataInstruction::Char(c) => EventType::SequenceItem(c),
    AutomataInstruction::Reset => EventType::SequenceReset,
  });
}

fn notify_success(event_notifier: &mut EventNotifier) {
  event_notifier.notify_with(|| EventType::FoundResults);
}

pub fn manage_automata(
  results_sender: &Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
  event_notifier: &mut EventNotifier,
  state: &Mutex<MainProcessState>,
) {
  for instruction in sequence_rec {
    notify_event(event_notifier, instruction);

    let put_result = state.lock().unwrap().automata.put(instruction);

    if let Some(results) = put_result {
      notify_success(event_notifier);

      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
