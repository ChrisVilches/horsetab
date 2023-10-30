use super::subcommands::{
  edit_subcommand, send_sequence_subcommand, show_subcommand, watch_sequences_subcommand,
};
use crate::{
  constants::{get_default_config_path, DEFAULT_PORT},
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

  #[command(about = "Send a sequence")]
  SendSequence {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: u32,

    #[arg(short, long)]
    sequence: String,
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

fn match_cli_subcommand(command: &Commands) -> Result<String> {
  match command {
    Commands::Serve { port, config_path } => {
      server::main::start(*port, config_path);
      Ok(String::new())
    }
    Commands::Edit { port } => edit_subcommand(*port),
    Commands::Show { port, raw } => show_subcommand(*port, *raw),
    Commands::SendSequence { port, sequence } => send_sequence_subcommand(*port, sequence),
    Commands::Watch { port } => watch_sequences_subcommand(*port),
  }
}

pub fn start_cli_app() {
  let cli = Cli::parse();

  if let Some(command) = &cli.command {
    let subcommand_result = match_cli_subcommand(command);

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
}
