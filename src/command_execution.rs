use crate::{
  constants::DEFAULT_INTERPRETER,
  logger::{log_stderr, log_stdout},
  util::parse_shebang_or_default,
};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::process::{Command, Stdio};

fn seconds_elapsed_since(date_time: DateTime<Local>) -> i64 {
  Local::now().timestamp() - date_time.timestamp()
}

// TODO: If I implement it like this, then I could extract the shebang from the first line
//       and call the shell accordingly. Is that too complex to manage????
//       I'd have to write down the full specifications. such as...
//       * It executes the entire script everytime a command is trigger
//       * It attemps to read the first line (shebang)
//       * Etc....
//       NOTE: implemented. Test, review, etc.
//       Also document (explain how all of this is executed). This can be in the readme, config file section
//       (explain what can be done in that section).
//       Also explain that the line cannot begin with a dot, so "source" must be "source" and not "."
pub fn spawn_process(pre_cmd: &str, cmd: &str) -> Result<()> {
  let start_time = Local::now();

  let full_command = format!("{pre_cmd}\n{cmd}");

  let interpreter = parse_shebang_or_default(pre_cmd, &DEFAULT_INTERPRETER);

  let mut child = Command::new(&interpreter[0])
    .args(&interpreter[1..])
    .arg(&full_command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .with_context(|| {
      format!("Cannot execute command using {interpreter:?}.\nCommand(s) executed: {full_command}")
    })?;

  let status = child.wait().expect("Should wait child");

  let elapsed_sec = seconds_elapsed_since(start_time);

  let stdout = child.stdout.unwrap();
  let stderr = child.stderr.unwrap();

  log_stdout(stdout, cmd, status, start_time, elapsed_sec);
  log_stderr(stderr, cmd, status, start_time, elapsed_sec);

  Ok(())
}
