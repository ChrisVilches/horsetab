use rdev::{listen, EventType};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::{
  click_sequence_detector::{ClickSequenceDetector, MouseClickKind},
  sequence_automata::AutomataInstruction,
};

const fn click_kind_to_instruction(click_kind: MouseClickKind) -> AutomataInstruction {
  match click_kind {
    MouseClickKind::Short => AutomataInstruction::Char('.'),
    MouseClickKind::Long => AutomataInstruction::Char('-'),
  }
}

fn handle_mouse_press(
  click_detector: &Mutex<ClickSequenceDetector>,
  seq_sender: &Sender<AutomataInstruction>,
) {
  let time_between_inputs = click_detector.lock().unwrap().click();

  if time_between_inputs > 500 {
    seq_sender
      .send(AutomataInstruction::Reset)
      .expect("Should send sequence instruction");
  }
}

fn handle_mouse_release(
  click_detector: &Mutex<ClickSequenceDetector>,
  seq_sender: &Sender<AutomataInstruction>,
) {
  let click_kind = click_detector.lock().unwrap().release();
  seq_sender
    .send(click_kind_to_instruction(click_kind))
    .expect("Should send mouse event message");
}

pub fn mouse_handler(seq_sender: Sender<AutomataInstruction>) {
  let click_detector = Arc::new(Mutex::new(ClickSequenceDetector::new(200)));

  let listen_result = listen(move |event| match event.event_type {
    EventType::ButtonPress(_) => handle_mouse_press(&click_detector, &seq_sender),
    EventType::ButtonRelease(_) => handle_mouse_release(&click_detector, &seq_sender),
    _ => {}
  });

  if let Err(error) = listen_result {
    eprintln!("Device error: {error:?}");
  }
}
