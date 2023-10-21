use std::time::Instant;

use crate::constants::MouseClickKind;

// TODO: It's not just a length detector anymore. It does more than that, so update the name
//       so it's a bit more generic. It also detects time between inputs (previous and current).

pub struct ClickLengthDetector {
  timestamp: Instant,
  long_ms: u128,
}

impl ClickLengthDetector {
  pub fn new(long_ms: u128) -> Self {
    Self {
      timestamp: Instant::now(),
      long_ms: long_ms,
    }
  }

  #[must_use]
  pub fn click(&mut self) -> u128 {
    let elapsed_since_last = self.timestamp.elapsed().as_millis();
    self.timestamp = Instant::now();

    elapsed_since_last
  }

  pub fn release(&mut self) -> MouseClickKind {
    let elapsed = self.timestamp.elapsed().as_millis();
    self.timestamp = Instant::now();
    if elapsed > self.long_ms {
      MouseClickKind::Long
    } else {
      MouseClickKind::Short
    }
  }
}
