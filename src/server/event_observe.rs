use std::{
  collections::HashMap,
  io::Write,
  net::{TcpListener, TcpStream},
  sync::{mpsc::Receiver, Mutex},
};

#[derive(Clone, Copy)]
pub enum EventType {
  SequenceItem(char),
  SequenceReset,
  FoundResults,
}

impl ToString for EventType {
  fn to_string(&self) -> String {
    match self {
      Self::SequenceItem(c) => format!("{c}"),
      Self::SequenceReset => "\n".to_owned(),
      Self::FoundResults => "* Match found\n".to_owned(),
    }
  }
}

pub fn notify_watch_observers(
  events_receiver: Receiver<EventType>,
  observers: &Mutex<HashMap<u16, TcpStream>>,
) {
  for event in events_receiver {
    let mut guard = observers.lock().unwrap();

    if guard.is_empty() {
      continue;
    }

    let msg = event.to_string();

    let keys: Vec<u16> = guard.keys().copied().collect();

    for key in keys {
      let stream = guard.get_mut(&key).expect("Should contain key");
      if stream.write(msg.as_bytes()).is_err() {
        guard.remove(&key);
      }
    }
  }
}

pub fn collect_watch_observers(
  tcp_listener: &TcpListener,
  observers: &Mutex<HashMap<u16, TcpStream>>,
) {
  for incoming_stream in tcp_listener.incoming() {
    match incoming_stream {
      Ok(stream) => {
        let port = stream.peer_addr().unwrap().port();
        observers.lock().unwrap().insert(port, stream);
      }
      Err(e) => {
        eprintln!("TCP error: {e}");
      }
    }
  }
}
