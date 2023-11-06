use crate::{
  ipc_tcp::{EventType, TcpAction, TcpActionResult},
  util::PayloadOverwriter,
};
use std::{
  collections::HashMap,
  io::Write,
  net::{TcpListener, TcpStream},
  sync::{mpsc::Receiver, Mutex},
};

pub fn notify_watch_observers(
  events_receiver: Receiver<EventType>,
  observers: &Mutex<HashMap<u16, TcpStream>>,
) {
  let mut payload = PayloadOverwriter::new();

  for event in events_receiver {
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

fn handle_tcp_watch(stream: TcpStream, observers: &Mutex<HashMap<u16, TcpStream>>) {
  let port = stream.peer_addr().unwrap().port();
  if let Err(e) = stream.shutdown(std::net::Shutdown::Read) {
    eprintln!("{e}");
  }

  observers.lock().unwrap().insert(port, stream);
}

fn respond_tcp_action_result(stream: &mut TcpStream, ok: bool) -> anyhow::Result<()> {
  let response = if ok {
    bincode::serialize(&TcpActionResult::Ok)
  } else {
    bincode::serialize(&TcpActionResult::Wrong)
  }?;

  stream.write_all(&response)?;
  Ok(())
}

fn handle_incoming_stream(mut stream: &mut TcpStream) -> anyhow::Result<TcpAction> {
  let tcp_action = bincode::deserialize_from(&mut stream);

  respond_tcp_action_result(stream, tcp_action.is_ok())?;

  if let Ok(action) = tcp_action {
    Ok(action)
  } else {
    anyhow::bail!("Incorrect TCP action");
  }
}

pub fn collect_watch_observers(
  tcp_listener: &TcpListener,
  observers: &Mutex<HashMap<u16, TcpStream>>,
) {
  for incoming_stream in tcp_listener.incoming() {
    match incoming_stream {
      Ok(mut stream) => match handle_incoming_stream(&mut stream) {
        Ok(TcpAction::Watch) => handle_tcp_watch(stream, observers),
        Err(e) => eprintln!("TCP stream handle error: {e}"),
      },
      Err(e) => {
        eprintln!("TCP error: {e}");
      }
    }
  }
}
