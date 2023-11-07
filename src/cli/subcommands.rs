use crate::constants::DEFAULT_COMMAND_CONFIG_FILE_CONTENT;
use crate::event_observe::EventType;
use crate::ipc_tcp::{connect_tcp, TcpAction};
use crate::{
  api_client::{self},
  cmd::Cmd,
};
use anyhow::Result;
use colored::Colorize;
use std::io::{BufReader, Read, Write};

pub fn show_subcommand(port: u16, raw: bool) -> Result<String> {
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

pub fn ps_subcommand(port: u16) -> Result<String> {
  api_client::get_ps(port)
}

pub fn edit_subcommand(port: u16) -> Result<String> {
  let current_config = api_client::get_current_config(port)?;

  let config_to_edit = if current_config.is_empty() {
    DEFAULT_COMMAND_CONFIG_FILE_CONTENT
  } else {
    &current_config
  };

  let new_content = edit::edit(config_to_edit)?;

  if new_content == current_config {
    Ok("No modification made".to_owned())
  } else {
    api_client::reinstall_commands(port, &new_content)
  }
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

fn newline_or_flush<W: Write>(
  event_type: EventType,
  out: &mut W,
  last_is_newline: &mut bool,
) -> Result<()> {
  if matches!(event_type, EventType::SequenceItem(_)) {
    out.flush()?;
    *last_is_newline = false;
  } else if !*last_is_newline {
    writeln!(out)?;
    *last_is_newline = true;
  }

  Ok(())
}

fn watch_sequences_print_formatted<R, W>(mut buf: BufReader<R>, mut out: W) -> Result<()>
where
  R: Read,
  W: Write,
{
  let mut last_is_newline = true;

  while let Ok(event_type) = bincode::deserialize_from(&mut buf) {
    match event_type {
      EventType::FoundResults => write!(out, "{}", " * Match found".yellow())?,
      EventType::SequenceItem(c) => write!(out, "{c}")?,
      EventType::SequenceReset => {}
    }

    newline_or_flush(event_type, &mut out, &mut last_is_newline)?;
  }

  Ok(())
}

pub fn watch_sequences_subcommand(port: u16) -> Result<String> {
  let tcp_port = api_client::get_tcp_port(port)?;
  let stream = connect_tcp(tcp_port, TcpAction::Watch)?;
  stream.shutdown(std::net::Shutdown::Write)?;
  watch_sequences_print_formatted(BufReader::new(stream), std::io::stdout())?;
  anyhow::bail!("Stopped getting data");
}

pub fn send_sequence_subcommand(port: u16, sequence: &str) -> Result<String> {
  api_client::send_sequence(port, sequence)?;
  Ok(String::new())
}

#[cfg(test)]
mod tests {
  use std::io::Cursor;

  use super::*;
  use test_case::test_case;

  fn found() -> String {
    " * Match found".yellow().to_string()
  }

  fn events_to_bytes(event_string: &str) -> Vec<u8> {
    event_string
      .chars()
      .map(|c| match c {
        'F' => EventType::FoundResults,
        'R' => EventType::SequenceReset,
        item => EventType::SequenceItem(item),
      })
      .flat_map(|ev| bincode::serialize(&ev).unwrap())
      .collect()
  }

  #[test_case("R..-..-F..-..-FR", &format!("..-..-{}\n..-..-{}\n", found(), found()))]
  #[test_case("R..-..-F..-..-R", &format!("..-..-{}\n..-..-\n", found()))]
  #[test_case("RRRRRRRRRRRRR", "")]
  #[test_case("RRRRRRRRR..-.RRR.-R.-RRRR.", "..-.\n.-\n.-\n.")]
  #[test_case("RRRRRRRRR..-.RRR.-R.-RRRR.RRRR", "..-.\n.-\n.-\n.\n")]
  #[test_case("R..FRR--F", &format!("..{}\n--{}\n", found(), found()))]
  #[test_case("R..FRR--FR", &format!("..{}\n--{}\n", found(), found()))]
  fn test_watch_sequences_print_formatted(event_string: &str, expected: &str) {
    let read = BufReader::new(Cursor::new(events_to_bytes(event_string)));

    let mut write = vec![];

    watch_sequences_print_formatted(read, &mut write).unwrap();

    let result = String::from_utf8_lossy(&write);
    assert!(!result.contains("\n\n"));
    assert!(!result.starts_with('\n'));
    assert_eq!(result, expected);
  }
}
