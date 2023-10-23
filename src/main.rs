#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::integer_division)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::string_to_string)]
#![deny(clippy::str_to_string)]
#![deny(clippy::try_err)]
#![deny(clippy::panic)]
#![deny(clippy::shadow_same)]
#![deny(clippy::shadow_reuse)]
#![deny(clippy::shadow_unrelated)]

mod click_sequence_detector;
mod cmd;
mod cmd_parser;
mod logger;
mod sequence_automata;
mod server;
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::StatusCode;

#[derive(Subcommand)]
enum Commands {
  #[command(about = "Start server process")]
  Serve {
    #[arg(short, long)]
    port: String,

    #[arg(short, long)]
    config_path: String,
  },

  #[command(about = "Edit commands")]
  Edit {
    #[arg(short, long)]
    port: String,
  },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

fn get_config_path_call(port: &str) -> Result<String> {
  let res =
    reqwest::blocking::get(format!("http://localhost:{port}/config-path"))?.error_for_status()?;
  Ok(res.text()?)
}

fn reinstall_call(port: &str) -> Result<String> {
  let client = reqwest::blocking::Client::new();
  let res = client
    .put(format!("http://localhost:{port}/re-install"))
    .send()?;

  match res.status() {
    StatusCode::OK => Ok(res.text()?),
    _ => Err(anyhow::anyhow!("{}", res.text()?)),
  }
}

fn main() {
  let cli = Cli::parse();

  if let Some(command) = &cli.command {
    match command {
      Commands::Serve { port, config_path } => server::controller::start(port, config_path),
      Commands::Edit { port } => {
        let config_path = get_config_path_call(port).expect("Should communicate with daemon");
        edit::edit_file(config_path).unwrap();

        match reinstall_call(port) {
          Ok(msg) => {
            println!("{msg}");
          }
          Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
          }
        }
      }
    }
  }
}
