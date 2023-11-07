use serde::{Deserialize, Serialize};

use crate::util::PayloadOverwriter;
use std::{collections::HashMap, io::Write, net::TcpStream, sync::Mutex};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum EventType {
  SequenceReset,
  FoundResults,
  SequenceItem(char),
}

pub fn notify_watch_observers<I, W>(events: I, observers: &Mutex<HashMap<u16, W>>)
where
  I: Iterator<Item = EventType>,
  W: Write,
{
  let mut payload = PayloadOverwriter::new();

  for event in events {
    let mut guard = observers.lock().unwrap();

    if guard.is_empty() {
      continue;
    }

    payload.overwrite_serialize(event).unwrap();

    for key in &guard.keys().copied().collect::<Vec<u16>>() {
      let w = guard.get_mut(key).expect("Should contain key");

      if w.write(&payload).is_err() {
        guard.remove(key);
      }
    }
  }
}

pub fn subscribe_watch_stream(stream: TcpStream, observers: &Mutex<HashMap<u16, TcpStream>>) {
  let port = stream.peer_addr().unwrap().port();
  if let Err(e) = stream.shutdown(std::net::Shutdown::Read) {
    eprintln!("{e}");
  }

  observers.lock().unwrap().insert(port, stream);
}
