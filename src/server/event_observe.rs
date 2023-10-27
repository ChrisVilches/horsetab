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
      Self::FoundResults => "*\n".to_owned(),
    }
  }
}
// TODO: Too many mutexes.. Even in the caller, it uses another mutex to protect the observers.
//       Can't I just pass the raw objects and let the internal mutexes protect the data?

pub struct EventNotifier {
  observers: Arc<Mutex<BTreeMap<String, File>>>,
}

pub struct EventSubscriber {
  observers: Arc<Mutex<BTreeMap<String, File>>>,
}

impl EventNotifier {
  pub fn notify_with(&mut self, f: impl Fn() -> EventType) {
    // TODO: Has to acquire this lock every time I click (even if there are no subscribers),
    //       plus the mutex for the wrapper of the notifier. Probably no way to fix this.
    //       Triage.

    if !self.observers.lock().unwrap().is_empty() {
      self.notify(f());
    }
  }

  fn unsubscribe(&self, files: &[String]) {
    let mut observers = self.observers.lock().unwrap();

    for file_to_remove in files {
      observers.remove(file_to_remove);
    }
  }

  fn notify(&mut self, event: EventType) {
    let content = event.to_string();

    let mut remove_files = Vec::<String>::new();

    let observers = self.observers.lock().unwrap();

    println!("Broadcasting to {} clients", observers.len());

    for (file_name, mut file) in &*observers {
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

    self.unsubscribe(&remove_files);
    // TODO: Kinda weird... this should be in the subscriber one, since it's its job to add/delete subscribers.
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
