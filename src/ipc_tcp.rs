use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  net::{TcpListener, TcpStream},
  sync::Mutex,
};

use crate::event_observe::subscribe_watch_stream;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TcpAction {
  Watch,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum TcpActionResult {
  Ok,
  Wrong,
}

impl From<bool> for TcpActionResult {
  fn from(value: bool) -> Self {
    if value {
      Self::Ok
    } else {
      Self::Wrong
    }
  }
}

static INCORRECT_TCP_ACTION_ERR: &str = "Incorrect TCP action";

pub fn connect_tcp(tcp_port: u16, action: TcpAction) -> Result<TcpStream> {
  let stream = TcpStream::connect(format!("localhost:{tcp_port}"))?;

  bincode::serialize_into(&stream, &action)?;

  let tcp_action_result = bincode::deserialize_from(&stream)?;

  if matches!(tcp_action_result, TcpActionResult::Ok) {
    Ok(stream)
  } else {
    anyhow::bail!(INCORRECT_TCP_ACTION_ERR);
  }
}

pub fn handle_tcp_action(mut stream: &mut TcpStream) -> anyhow::Result<TcpAction> {
  let tcp_action = bincode::deserialize_from(&mut stream).with_context(|| INCORRECT_TCP_ACTION_ERR);
  let tcp_action_result = TcpActionResult::from(tcp_action.is_ok());
  bincode::serialize_into(stream, &tcp_action_result)?;
  tcp_action
}

pub fn start_tcp_server(tcp_listener: &TcpListener, observers: &Mutex<HashMap<u16, TcpStream>>) {
  for incoming_stream in tcp_listener.incoming() {
    match incoming_stream {
      Ok(mut stream) => match handle_tcp_action(&mut stream) {
        Ok(TcpAction::Watch) => subscribe_watch_stream(stream, observers),
        Err(e) => eprintln!("TCP stream handle error: {e}"),
      },
      Err(e) => {
        eprintln!("TCP error: {e}");
      }
    }
  }
}
