use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::StatusCode;

use crate::server;

#[derive(Subcommand)]
pub enum Commands {
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
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,
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

// TODO: I feel like the initial configuration file (when the user installs the app) should have
//       some comments to explain stuff. But note that comments aren't handled yet (they throw error when
//       parsed).

// TODO: Which file format would be the best so that Vim and other editors choose the best formatting/colors?

// TODO: This is a bit bad. If the command is executed from two shells, the file will be opened using
//       Vim in my case (a sensible guess for other users as well) but it will trigger a warning.
//       It will also tell the user where the file is located. A better way to do this would be to simply
//       1. GET the current content (new API needed)
//       2. Open a temp file
//       3. Edit and save the temp file
//       4. POST the new contents to the server (new API needed)
//       5. Make the server save the new content and re-read to install
fn edit_subcommand(port: &str) {
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

fn match_cli_command(cli: &Cli) {
  if let Some(command) = &cli.command {
    match command {
      Commands::Serve { port, config_path } => server::main::start(port, config_path),
      Commands::Edit { port } => edit_subcommand(port),
    }
  }
}

pub fn start_cli_app() {
  let cli = Cli::parse();
  match_cli_command(&cli);
}
