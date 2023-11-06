use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum TcpAction {
  Watch,
}

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub enum TcpActionResult {
  Ok,
  Wrong,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum EventType {
  SequenceReset,
  FoundResults,
  SequenceItem(char),
}
