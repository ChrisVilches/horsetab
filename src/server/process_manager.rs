use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::io::Write;
use std::process::{ChildStderr, ChildStdout};
use std::sync::{Arc, Mutex};
use std::{
  io::BufReader,
  process::{Child, Command, ExitStatus, Stdio},
};
use tempfile::NamedTempFile;

use crate::{
  logger::{log_stdout, redirect_output},
  util::seconds_elapsed,
};

struct Process {
  cmd: String,
  start_time: DateTime<Local>,
  end_time: Option<DateTime<Local>>,
  pid: u32,
  status: Option<ExitStatus>,
}

impl Process {
  fn new(pid: u32, cmd: &str) -> Self {
    Self {
      cmd: cmd.to_owned(),
      start_time: Local::now(),
      end_time: None,
      pid,
      status: None,
    }
  }
}

macro_rules! process_4col_format {
  ($pid:expr, $time:expr, $status:expr, $cmd:expr) => {
    format!("{:<15}{:<15}{:<25}{}", $pid, $time, $status, $cmd)
  };
}

impl ToString for Process {
  fn to_string(&self) -> String {
    let elapsed = seconds_elapsed(self.start_time, self.end_time);

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

fn handle_child_exit(
  child: &Mutex<Child>,
  process_map: &Mutex<HashMap<u32, Process>>,
  pid: u32,
  start_time: DateTime<Local>,
) {
  let status = child.lock().unwrap().wait().expect("Should wait child");
  let end_time = Some(Local::now());

  if let Some(process) = process_map.lock().unwrap().get_mut(&pid) {
    process.status = Some(status);
    process.end_time = end_time;
  }

  let elapsed_sec = seconds_elapsed(start_time, end_time);

  log_stdout(
    pid,
    &format!("Done in {elapsed_sec}s{}", format_exit_status(status)),
  );
}

fn get_child_information(child: &Mutex<Child>) -> (u32, ChildStdout, ChildStderr) {
  let mut child_guard = child.lock().unwrap();
  let pid = child_guard.id();
  let stdout = child_guard.stdout.take().unwrap();
  let stderr = child_guard.stderr.take().unwrap();
  (pid, stdout, stderr)
}

fn handle_child(
  child: &Mutex<Child>,
  start_time: DateTime<Local>,
  initial_cmd: &str,
  process_map: &Mutex<HashMap<u32, Process>>,
) {
  let (pid, stdout, stderr) = get_child_information(child);

  log_stdout(pid, &format!("Started {initial_cmd}"));

  std::thread::scope(|scope| {
    scope.spawn(|| redirect_output(BufReader::new(stdout), pid, true));
    scope.spawn(|| redirect_output(BufReader::new(stderr), pid, false));
    scope.spawn(|| handle_child_exit(child, process_map, pid, start_time));
  });
}

fn spawn_process(
  interpreter: &str,
  pre_script: &str,
  cmd: String,
  process_map: Arc<Mutex<HashMap<u32, Process>>>,
) -> Result<Process> {
  let child = create_child(interpreter, pre_script, &cmd)?;

  let pid = child.id();

  let wrapped_child = Arc::new(Mutex::new(child));

  let process = Process::new(pid, &cmd);

  std::thread::spawn(move || {
    handle_child(&wrapped_child, process.start_time, &cmd, &process_map);
    std::thread::sleep(std::time::Duration::from_secs(5));
    process_map.lock().unwrap().remove(&pid);
  });

  Ok(process)
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

  pub fn format_information(&self) -> String {
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

    let process = spawn_process(interpreter, pre_script, cmd.into(), process_map)?;

    let pid = process.pid;

    self.process_map.lock().unwrap().insert(pid, process);

    Ok(pid)
  }
}
