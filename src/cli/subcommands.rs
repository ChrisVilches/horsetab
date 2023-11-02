use std::io::{BufReader, Read, Write};
use std::process::{ChildStdout, Command, Stdio};

use crate::constants::DEFAULT_COMMAND_CONFIG_FILE_CONTENT;
use crate::util::effectful_format_bytes_merge_newlines;
use crate::{
  api_client::{self},
  cmd::Cmd,
};
use anyhow::Result;
use colored::Colorize;

pub fn show_subcommand(port: u32, raw: bool) -> Result<String> {
  let current_config = api_client::get_current_installed_commands(port);

  #[allow(clippy::option_if_let_else)]
  match current_config {
    Ok(text) => {
      if raw {
        Ok(text)
      } else {
        Ok(format_commands(&text))
      }
    }
    Err(_) => current_config,
  }
}

pub fn ps_subcommand(port: u32) -> Result<String> {
  api_client::get_ps(port)
}

pub fn edit_subcommand(port: u32) -> Result<String> {
  let current_config = api_client::get_current_config(port)?;

  let config_to_edit = if current_config.is_empty() {
    DEFAULT_COMMAND_CONFIG_FILE_CONTENT
  } else {
    &current_config
  };

  let new_content = edit::edit(config_to_edit)?;

  api_client::reinstall_commands(port, &new_content)
}

fn format_commands(commands_text: &str) -> String {
  commands_text
    .split('\n')
    .filter(|s| !s.is_empty())
    .map(|s| Cmd::parse(s).expect("Should have correct format"))
    .map(|cmd| format!("{}\t{}", cmd.sequence.yellow().bold(), cmd.command))
    .collect::<Vec<String>>()
    .join("\n")
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

fn print_from_buf_reader<R, W>(mut buf: BufReader<R>, mut out: W)
where
  R: Read,
  W: Write,
{
  let mut last_char = b'\n';
  let mut result = [0; 30];

  loop {
    let n_read = buf.read(&mut result).unwrap();
    if n_read == 0 {
      break;
    }

    let content_to_print = effectful_format_bytes_merge_newlines(&mut result, n_read, last_char);

    if !content_to_print.is_empty() {
      out.write_all(content_to_print).unwrap();
      out.flush().unwrap();
      last_char = *content_to_print.last().unwrap();
    }
  }
}

pub fn watch_sequences_subcommand(port: u32) -> Result<String> {
  let path = create_named_pipe()?;

  let child_stdout = get_file_stdout_stream(&path)?;

  api_client::watch_sequences(port, &path)?;

  print_from_buf_reader(child_stdout, std::io::stdout());

  println!();
  anyhow::bail!("Stopped getting data");
}

pub fn send_sequence_subcommand(port: u32, sequence: &str) -> Result<String> {
  api_client::send_sequence(port, sequence)?;
  Ok(String::new())
}
