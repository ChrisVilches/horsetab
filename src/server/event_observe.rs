use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
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
      Self::FoundResults => "* Match found\n".to_owned(),
    }
  }
}
pub struct EventNotifier {
  observers: Arc<Mutex<BTreeMap<String, File>>>,
}

pub struct EventSubscriber {
  observers: Arc<Mutex<BTreeMap<String, File>>>,
}

impl EventNotifier {
  pub fn notify_with(&mut self, f: impl Fn() -> EventType) {
    if !self.observers.lock().unwrap().is_empty() {
      self.notify(f());
    }
  }

  fn notify(&mut self, event: EventType) {
    let content = event.to_string();

    let mut remove_files = Vec::<String>::new();

    let mut observers = self.observers.lock().unwrap();

    for (file_name, mut file) in &*observers {
      let result = file.write(content.as_bytes());

      if let Err(err) = result {
        match err.kind() {
          std::io::ErrorKind::BrokenPipe => {}
          e => eprintln!("Unhandled error while notifying: {e}"),
        }

        remove_files.push(file_name.clone());
      }
    }

    for file_to_remove in remove_files {
      observers.remove(&file_to_remove);
      if let Err(err) = fs::remove_file(file_to_remove) {
        eprintln!("Error while removing file: {err}");
      }
    }
  }
}

impl EventSubscriber {
  pub fn subscribe(&mut self, output_file_path: &str) {
    match unix_named_pipe::open_write(output_file_path) {
      Ok(file) => {
        self
          .observers
          .lock()
          .unwrap()
          .insert(output_file_path.to_owned(), file);
      }
      Err(err) => eprintln!("{err}"),
    }
  }
}

pub fn make_event_observer() -> (EventSubscriber, EventNotifier) {
  let observers = Arc::new(Mutex::new(BTreeMap::<String, File>::new()));
  let ref1 = Arc::clone(&observers);
  let ref2 = Arc::clone(&observers);
  let subscriber = EventSubscriber { observers: ref1 };
  let notifier = EventNotifier { observers: ref2 };
  (subscriber, notifier)
}
