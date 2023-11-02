use std::{
  collections::HashSet,
  sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::{
  io::BufReader,
  process::{Child, Command, ExitStatus, Stdio},
};

use crate::{
  logger::{log_stdout, redirect_output},
  util::{create_temp_file, seconds_elapsed_since},
};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Process {
  cmd: String,
  start_time: DateTime<Local>,
  pid: u32,
}

macro_rules! process_4col_format {
  ($pid:expr, $time:expr, $cmd:expr) => {
    format!("{:<15}{:<15}{}", $pid, $time, $cmd)
  };
}

impl ToString for Process {
  fn to_string(&self) -> String {
    let elapsed = seconds_elapsed_since(self.start_time);
    process_4col_format!(self.pid, elapsed, self.cmd)
  }
}

fn format_exit_status(exit_status: ExitStatus) -> String {
  if exit_status.success() {
    String::new()
  } else {
    format!(" ({exit_status})")
  }
}

fn create_child(pre_script: &str, cmd: &str) -> Result<Child> {
  let full_command = format!("{pre_script}\n{cmd}");

  let temp_path = create_temp_file("horsetab-exec", &full_command, 10)?;

  let child = Command::new(temp_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .with_context(|| format!("Cannot execute command:\n{full_command}"));

  child
}

fn handle_child(mut child: Child, start_time: DateTime<Local>, initial_cmd: &str) {
  let pid = child.id();

  log_stdout(pid, &format!("Started {initial_cmd}"));

  let stdout = child.stdout.take().unwrap();
  let stderr = child.stderr.take().unwrap();

  std::thread::scope(|scope| {
    scope.spawn(|| redirect_output(BufReader::new(stdout), pid, true));
    scope.spawn(|| redirect_output(BufReader::new(stderr), pid, false));
  });

  let status = child.wait().expect("Should wait child");
  let elapsed_sec = seconds_elapsed_since(start_time);

  log_stdout(
    pid,
    &format!("Done in {elapsed_sec}s{}", format_exit_status(status)),
  );
}

fn spawn_process(
  start_time: DateTime<Local>,
  pre_script: &str,
  cmd: &str,
  process_set: Arc<Mutex<HashSet<Process>>>,
) -> Result<Process> {
  let cmd_clone = cmd.to_owned();

  let child = create_child(pre_script, &cmd_clone)?;
  let pid = child.id();

  let process = Process {
    cmd: cmd.to_owned(),
    start_time,
    pid,
  };

  let process_clone = process.clone();

  std::thread::spawn(move || {
    handle_child(child, start_time, &cmd_clone);
    process_set.lock().unwrap().remove(&process_clone);
  });

  Ok(process)
}

pub struct ProcessManager {
  process_set: Arc<Mutex<HashSet<Process>>>,
}

impl ProcessManager {
  pub fn new() -> Self {
    Self {
      process_set: Arc::new(Mutex::new(HashSet::new())),
    }
  }

  fn format_process_lines(&self) -> String {
    self
      .process_set
      .lock()
      .unwrap()
      .iter()
      .map(std::string::ToString::to_string)
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn format_information(&self) -> String {
    let header = process_4col_format!("PID", "TIME (s)", "COMMAND");

    [header, self.format_process_lines()]
      .iter()
      .filter(|s| !s.is_empty())
      .cloned()
      .collect::<Vec<String>>()
      .join("\n")
  }

  pub fn start(&self, pre_script: &str, cmd: &str) -> Result<u32> {
    let process_set = Arc::clone(&self.process_set);

    let process = spawn_process(Local::now(), pre_script, cmd, process_set)?;

    let pid = process.pid;

    self.process_set.lock().unwrap().insert(process);

    Ok(pid)
  }
}
