use std::time::Instant;

#[derive(Copy, Clone)]
pub enum MouseClickKind {
  Short,
  Long,
}

pub struct ClickSequenceDetector {
  timestamp: Instant,
  long_ms: u128,
}

impl ClickSequenceDetector {
  pub fn new(long_ms: u128) -> Self {
    Self {
      timestamp: Instant::now(),
      long_ms,
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
