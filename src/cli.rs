use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};

use crate::{
  api_client::{self},
  cmd::{parse_command, Cmd},
  constants::{get_default_config_path, DEFAULT_PORT},
  server,
  util::clean_command_lines,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Start server process")]
  Serve {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,

    #[arg(short, long, default_value_t = get_default_config_path())]
    config_path: String,
  },

  #[command(about = "Show current commands")]
  Show {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,

    #[arg(short, long, default_value_t = false)]
    raw: bool,
  },

  #[command(about = "Edit commands")]
  Edit {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,
  },

  #[command(about = "Watch sequences")]
  Watch {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,
  },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

// TODO: I feel like the initial configuration file (when the user installs the app) should have
//       some comments to explain stuff. But note that comments aren't handled yet (they throw error when
//       parsed).
//       Move this to issues on Github since it's hard to implement. This would be a future feature (not in this phase scope.)

// TODO: Which file format would be the best so that Vim and other editors choose the best formatting/colors?
//       Probably also for a different phase.

fn edit_subcommand(port: u32) -> Result<String> {
  let current_config = api_client::get_current_config(port)?;
  let new_content = edit::edit(current_config)?;
  api_client::reinstall_commands(port, &new_content)
}

fn text_to_commands(text: &str) -> (Vec<Cmd>, usize) {
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

fn listen_sequences_blocking(path: &str, after_opened_file_callback: impl Fn()) -> Result<()> {
  let mut child = Command::new("cat")
    .arg(path)
    .stdout(Stdio::piped())
    .spawn()?;

  let mut has_content = false;
  let out = BufReader::new(child.stdout.take().unwrap());

  after_opened_file_callback();

  for byte in out.bytes().flatten() {
    if !has_content && byte == b'\n' {
      continue;
    }

    std::io::stdout().write_all(&[byte]).unwrap();
    std::io::stdout().flush().unwrap();
    has_content = true;
  }

  Ok(())
}

fn watch_sequences_subcommand(port: u32) -> Result<String> {
  let path = create_named_pipe()?;

  listen_sequences_blocking(&path, || {
    api_client::watch_sequences(port, &path).unwrap();
  })?;

  println!();
  anyhow::bail!("Server exited");
}

fn match_cli_subcommand(command: &Commands) -> Result<String> {
  match command {
    Commands::Serve { port, config_path } => {
      server::main::start(*port, config_path);
      Ok(String::new())
    }
    Commands::Edit { port } => edit_subcommand(*port),
    Commands::Show { port, raw } => {
      let current_config = api_client::get_current_config(*port);

      #[allow(clippy::option_if_let_else)]
      match current_config {
        Ok(text) => {
          if *raw {
            Ok(text)
          } else {
            let (cmds, failed) = text_to_commands(&text);
            Ok(format_commands_list(cmds, failed))
          }
        }
        Err(_) => current_config,
      }
    }
    Commands::Watch { port } => watch_sequences_subcommand(*port),
  }
}

pub fn start_cli_app() {
  let cli = Cli::parse();

  if let Some(command) = &cli.command {
    let subcommand_result = match_cli_subcommand(command);

    match subcommand_result {
      Ok(msg) => {
        println!("{msg}");
      }
      Err(err) => {
        eprintln!("{}", err.to_string().red());
        std::process::exit(1);
      }
    }
  }
}
