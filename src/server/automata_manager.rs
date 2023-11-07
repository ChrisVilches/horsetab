use super::global_context::MainProcessState;
use crate::event_observe::EventType;
use crate::sequence_automata::AutomataInstruction;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

static SEND_ERR: &str = "Should send event";

fn notify_instruction(events_sender: &Sender<EventType>, instruction: AutomataInstruction) {
  let event = match instruction {
    AutomataInstruction::Char(c) => EventType::SequenceItem(c),
    AutomataInstruction::Reset => EventType::SequenceReset,
  };

  events_sender.send(event).expect(SEND_ERR);
}

pub fn manage_automata(
  results_sender: &Sender<usize>,
  sequence_rec: Receiver<AutomataInstruction>,
  events_sender: &Sender<EventType>,
  state: &Mutex<MainProcessState>,
) {
  for instruction in sequence_rec {
    notify_instruction(events_sender, instruction);

    let put_result = state.lock().unwrap().automata.put(instruction);

    if let Some(results) = put_result {
      events_sender.send(EventType::FoundResults).expect(SEND_ERR);

      for result_id in results {
        results_sender
          .send(result_id)
          .expect("Result should be sent");
      }
    }
  }
}
