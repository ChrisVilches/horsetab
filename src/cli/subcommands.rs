use std::io::{BufReader, Read, Write};
use std::process::{ChildStdout, Command, Stdio};

use crate::{
  api_client::{self},
  cmd::{parse_command, Cmd},
  util::clean_command_lines,
};
use anyhow::Result;
use colored::Colorize;

pub fn show_subcommand(port: u32, raw: bool) -> Result<String> {
  let current_config = api_client::get_current_config(port);

  #[allow(clippy::option_if_let_else)]
  match current_config {
    Ok(text) => {
      if raw {
        Ok(text)
      } else {
        let (cmds, failed) = text_to_commands(&text);
        Ok(format_commands_list(cmds, failed))
      }
    }
    Err(_) => current_config,
  }
}

pub fn edit_subcommand(port: u32) -> Result<String> {
  let current_config = api_client::get_current_config(port)?;
  let new_content = edit::edit(current_config)?;
  api_client::reinstall_commands(port, &new_content)
}

pub fn text_to_commands(text: &str) -> (Vec<Cmd>, usize) {
  let mut failed = 0;
  let mut commands = vec![];

  for line in clean_command_lines(text.lines()) {
    parse_command(&line).map_or_else(|_| failed += 1, |cmd| commands.push(cmd));
  }

  (commands, failed)
}

fn format_commands_list(commands: Vec<Cmd>, failed: usize) -> String {
  let mut result: Vec<String> = vec![];

  if failed > 0 {
    result.push(
      format!("{failed} command(s) failed to parse")
        .red()
        .to_string(),
    );
    result.push(String::new());
  }

  for cmd in commands {
    result.push(format!("{}\t{}", cmd.sequence.yellow().bold(), cmd.command));
  }

  result.join("\n")
}

fn create_named_pipe() -> Result<String> {
  let id = nanoid::nanoid!();
  let path = format!("/tmp/horsetab-{id}");
  unix_named_pipe::create(&path, Some(0o660))?;
  Ok(path)
}

fn get_file_stdout_stream(path: &str) -> Result<BufReader<ChildStdout>> {
  let mut child = Command::new("cat")
    .arg(path)
    .stdout(Stdio::piped())
    .spawn()?;

  Ok(BufReader::new(child.stdout.take().unwrap()))
}

fn print_from_buf_reader(buf: BufReader<ChildStdout>) {
  let mut has_content = false;

  for byte in buf.bytes().flatten() {
    if !has_content && byte == b'\n' {
      continue;
    }

    std::io::stdout().write_all(&[byte]).unwrap();
    std::io::stdout().flush().unwrap();
    has_content = true;
  }
}

pub fn watch_sequences_subcommand(port: u32) -> Result<String> {
  let path = create_named_pipe()?;

  let out = get_file_stdout_stream(&path)?;

  api_client::watch_sequences(port, &path).unwrap();

  print_from_buf_reader(out);

  println!();
  anyhow::bail!("Server exited");
}
