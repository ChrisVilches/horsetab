use crossbeam::{channel::Sender, sync::Parker};
use device_query::{CallbackGuard, DeviceEvents, DeviceState};
use std::sync::{Arc, Mutex};

use crate::{
  click_sequence_detector::{ClickSequenceDetector, MouseClickKind},
  sequence_automata::AutomataInstruction,
};

fn listen_mouse_down(
  device: &DeviceState,
  seq_sender: Sender<AutomataInstruction>,
  click_detector: Arc<Mutex<ClickSequenceDetector>>,
) -> CallbackGuard<impl Fn(&usize)> {
  device.on_mouse_down(move |_| {
    let time_between_inputs = click_detector.lock().unwrap().click();

    if time_between_inputs > 500 {
      seq_sender
        .send(AutomataInstruction::Reset)
        .expect("Should send sequence instruction");
    }
  })
}

const fn click_kind_to_instruction(click_kind: MouseClickKind) -> AutomataInstruction {
  match click_kind {
    MouseClickKind::Short => AutomataInstruction::Char('0'),
    MouseClickKind::Long => AutomataInstruction::Char('1'),
  }
}

fn listen_mouse_up(
  device: &DeviceState,
  seq_sender: Sender<AutomataInstruction>,
  click_detector: Arc<Mutex<ClickSequenceDetector>>,
) -> CallbackGuard<impl Fn(&usize)> {
  device.on_mouse_up(move |_| {
    let click_kind = click_detector.lock().unwrap().release();
    seq_sender
      .send(click_kind_to_instruction(click_kind))
      .expect("Should send mouse event message");
  })
}

pub fn mouse_handler(seq_sender: Sender<AutomataInstruction>) {
  let device = DeviceState::new();
  let click_detector = Arc::new(Mutex::new(ClickSequenceDetector::new(200)));

  let _guard1 = listen_mouse_down(&device, seq_sender.clone(), Arc::clone(&click_detector));
  let _guard2 = listen_mouse_up(&device, seq_sender, Arc::clone(&click_detector));

  Parker::new().park();
}
