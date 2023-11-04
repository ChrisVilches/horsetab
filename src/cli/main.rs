use super::subcommands::{
  edit_subcommand, ps_subcommand, send_sequence_subcommand, show_subcommand,
  watch_sequences_subcommand,
};
use crate::{
  constants::{get_default_config_path, DEFAULT_INTERPRETER, DEFAULT_PORT},
  server,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Start server process")]
  Serve {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(short, long, default_value_t = get_default_config_path())]
    config_path: String,

    #[arg(short, long, default_value_t = DEFAULT_INTERPRETER.to_owned())]
    interpreter: String,
  },

  #[command(about = "Show current commands")]
  Show {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(short, long, default_value_t = false)]
    raw: bool,
  },

  #[command(about = "Edit commands")]
  Edit {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
  },

  #[command(about = "Send a sequence")]
  SendSequence {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(
      short,
      long,
      help = "A morse sequence (use `-s=--` to avoid treating `--` as end of options)"
    )]
    sequence: String,
  },

  #[command(about = "Watch sequences")]
  Watch {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
  },

  #[command(about = "Display status information about processes")]
  Ps {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u16,
  },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

fn match_cli_subcommand(command: &Commands) -> Result<String> {
  match command {
    Commands::Serve {
      port,
      config_path,
      interpreter,
    } => {
      server::main::start(*port, config_path, interpreter);
      Ok(String::new())
    }
    Commands::Edit { port } => edit_subcommand(*port),
    Commands::Show { port, raw } => show_subcommand(*port, *raw),
    Commands::SendSequence { port, sequence } => send_sequence_subcommand(*port, sequence),
    Commands::Watch { port } => watch_sequences_subcommand(*port),
    Commands::Ps { port } => ps_subcommand(*port),
  }
}

fn handle_subcommand_result(subcommand_result: Result<String, anyhow::Error>) {
  match subcommand_result {
    Ok(msg) => {
      if !msg.is_empty() {
        println!("{msg}");
      }
    }
    Err(err) => {
      let err_msg = err.to_string();

      if !err_msg.is_empty() {
        eprintln!("{}", err_msg.red());
      }

      std::process::exit(1);
    }
  }
}

pub fn start_cli_app() {
  let cli = Cli::parse();

  if let Some(command) = &cli.command {
    let subcommand_result = match_cli_subcommand(command);

    handle_subcommand_result(subcommand_result);
  }
}
