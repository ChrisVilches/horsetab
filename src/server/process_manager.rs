use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::io::Write;
use std::process::{ChildStderr, ChildStdout};
use std::sync::{Arc, Mutex, MutexGuard};
use std::{
  io::BufReader,
  process::{Child, Command, ExitStatus, Stdio},
};
use tempfile::NamedTempFile;

use crate::{
  logger::{log_stdout, redirect_output},
  util::seconds_elapsed_since,
};

struct Process {
  cmd: String,
  start_time: DateTime<Local>,
  pid: u32,
  child: Arc<Mutex<Child>>,
  status: Option<ExitStatus>,
}

macro_rules! process_4col_format {
  ($pid:expr, $time:expr, $status:expr, $cmd:expr) => {
    format!("{:<15}{:<15}{:<25}{}", $pid, $time, $status, $cmd)
  };
}

impl ToString for Process {
  fn to_string(&self) -> String {
    let elapsed = seconds_elapsed_since(self.start_time);

    let status_str = self
      .status
      .map_or_else(|| "Running".to_owned(), |status| status.to_string());

    process_4col_format!(self.pid, elapsed, status_str, self.cmd)
  }
}

fn format_exit_status(exit_status: ExitStatus) -> String {
  if exit_status.success() {
    String::new()
  } else {
    format!(" ({exit_status})")
  }
}

fn create_child(interpreter: &str, pre_script: &str, cmd: &str) -> Result<Child> {
  let full_command = format!("{pre_script}\n{cmd}\n");

  let file = Arc::new(Mutex::new(NamedTempFile::new()?));
  write!(file.lock().unwrap(), "{full_command}").unwrap();

  let child = Command::new(interpreter)
    .arg(file.lock().unwrap().path())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .with_context(|| format!("({interpreter}) Cannot execute:\n{full_command}"));

  std::thread::spawn(|| {
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(file);
  });

  child
}

fn get_output(mut child: MutexGuard<'_, Child>) -> (ChildStdout, ChildStderr) {
  (child.stdout.take().unwrap(), child.stderr.take().unwrap())
}

fn handle_child(child: &Mutex<Child>, start_time: DateTime<Local>, initial_cmd: &str) {
  let child_guard = child.lock().unwrap();
  let pid = child_guard.id();
  let (stdout, stderr) = get_output(child_guard);

  log_stdout(pid, &format!("Started {initial_cmd}"));

  std::thread::scope(|scope| {
    scope.spawn(|| redirect_output(BufReader::new(stdout), pid, true));
    scope.spawn(|| redirect_output(BufReader::new(stderr), pid, false));
  });

  let status = child.lock().unwrap().wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  log_stdout(
    pid,
    &format!("Done in {elapsed_sec}s{}", format_exit_status(status)),
  );
}

fn spawn_process(
  interpreter: &str,
  start_time: DateTime<Local>,
  pre_script: &str,
  cmd: String,
  process_map: Arc<Mutex<HashMap<u32, Process>>>,
) -> Result<Process> {
  let child = create_child(interpreter, pre_script, &cmd)?;

  let pid = child.id();

  let wrapped_child = Arc::new(Mutex::new(child));

  let process = Process {
    cmd: cmd.clone(),
    start_time,
    pid,
    child: Arc::clone(&wrapped_child),
    status: None,
  };

  std::thread::spawn(move || {
    handle_child(&wrapped_child, start_time, &cmd);
    process_map.lock().unwrap().remove(&pid);
  });

  Ok(process)
}

pub fn start_garbage_collection(
  process_manager: Arc<Mutex<ProcessManager>>,
  interval: std::time::Duration,
) {
  std::thread::spawn(move || loop {
    if let Err(e) = process_manager.lock().unwrap().garbage_collect() {
      eprintln!("{e}");
    }
    std::thread::sleep(interval);
  });
}

fn garbage_collect_one_process(process: &mut Process, status: ExitStatus) {
  let msg = format!(
    "Garbage collector: Process (PID {}) finished abnormally{}",
    process.pid,
    format_exit_status(status)
  );
  log_stdout(0, &msg);
  process.status = Some(status);
}

fn check_process_finished(process: &Process) -> Result<Option<ExitStatus>> {
  process
    .child
    .lock()
    .unwrap()
    .try_wait()
    .with_context(|| format!("Cannot wait child (PID {})", process.pid))
}

pub struct ProcessManager {
  process_map: Arc<Mutex<HashMap<u32, Process>>>,
}

impl ProcessManager {
  pub fn new() -> Self {
    Self {
      process_map: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  fn garbage_collect(&mut self) -> Result<()> {
    let mut process_map = self.process_map.lock().unwrap();

    let process_without_exit_status = process_map
      .iter_mut()
      .filter(|(_, process)| process.status.is_none());

    for (_, process) in process_without_exit_status {
      if let Some(status) = check_process_finished(process)? {
        garbage_collect_one_process(process, status);
      }
    }

    Ok(())
  }

  fn format_process_lines(&self) -> String {
    self
      .process_map
      .lock()
      .unwrap()
      .values()
      .map(std::string::ToString::to_string)
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn format_information(&mut self) -> String {
    let header = process_4col_format!("PID", "TIME (s)", "STATUS", "COMMAND");

    [header, self.format_process_lines()]
      .iter()
      .filter(|s| !s.is_empty())
      .cloned()
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn start(&self, interpreter: &str, pre_script: &str, cmd: &str) -> Result<u32> {
    let process_map = Arc::clone(&self.process_map);

    let process = spawn_process(
      interpreter,
      Local::now(),
      pre_script,
      cmd.into(),
      process_map,
    )?;

    let pid = process.pid;

    self.process_map.lock().unwrap().insert(pid, process);

    Ok(pid)
  }
}
