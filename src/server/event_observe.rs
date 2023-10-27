use std::io::Write;
use std::{collections::BTreeMap, fs::File};

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
      Self::FoundResults => "*\n".to_owned(),
    }
  }
}

pub struct EventObserver {
  observers: BTreeMap<String, File>,
}

impl EventObserver {
  pub const fn new() -> Self {
    Self {
      observers: BTreeMap::new(),
    }
  }

  pub fn observe(&mut self, output_file_path: &str) {
    match unix_named_pipe::open_write(output_file_path) {
      Ok(file) => {
        self.observers.insert(output_file_path.to_owned(), file);
      }
      Err(err) => eprintln!("{err}"),
    }
  }

  pub fn notify_with(&mut self, f: impl Fn() -> EventType) {
    if !self.observers.is_empty() {
      self.notify(f())
    }
  }

  fn notify(&mut self, event: EventType) {
    let content = event.to_string();

    let mut remove_files = Vec::<String>::new();

    println!("Broadcasting to {} clients", self.observers.len());

    for (file_name, mut file) in &self.observers {
      let result = file.write(content.as_bytes());

      if let Err(err) = result {
        match err.kind() {
          std::io::ErrorKind::BrokenPipe => {
            remove_files.push(file_name.clone());
          }
          e => {
            eprintln!("{e}");
          }
        }
      }
    }

    for file_to_remove in remove_files {
      self.observers.remove(&file_to_remove);
    }
  }
}
