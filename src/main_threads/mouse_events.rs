use std::sync::{Arc, Mutex};

use crate::constants::MouseClickKind;
use crossbeam::{channel::Sender, sync::Parker};
use device_query::{CallbackGuard, DeviceEvents, DeviceState};

use crate::click_length_detector::ClickLengthDetector;

fn listen_mouse_down(
  device: &DeviceState,
  seq_sender: Sender<MouseClickKind>,
  click_detector: Arc<Mutex<ClickLengthDetector>>,
) -> CallbackGuard<impl Fn(&usize)> {
  device.on_mouse_down(move |_| {
    let time_between_inputs = click_detector.lock().unwrap().click();

    if time_between_inputs > 500 {
      seq_sender
        .send(MouseClickKind::Reset)
        .expect("Should send sequence instruction");
    }
  })
}

fn listen_mouse_up(
  device: &DeviceState,
  seq_sender: Sender<MouseClickKind>,
  click_detector: Arc<Mutex<ClickLengthDetector>>,
) -> CallbackGuard<impl Fn(&usize)> {
  device.on_mouse_up(move |_| {
    let click_kind = click_detector.lock().unwrap().release();
    seq_sender
      .send(click_kind)
      .expect("Should send mouse event message");
  })
}

pub fn listen_mouse_events(seq_sender: Sender<MouseClickKind>) {
  let device = DeviceState::new();
  let click_detector = Arc::new(Mutex::new(ClickLengthDetector::new(200)));

  let _guard = listen_mouse_down(&device, seq_sender.clone(), Arc::clone(&click_detector));
  let _guard = listen_mouse_up(&device, seq_sender.clone(), Arc::clone(&click_detector));

  Parker::new().park();
}
