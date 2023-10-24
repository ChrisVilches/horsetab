use crate::{api_client, server};
use anyhow::Result;
use clap::{Parser, Subcommand};

// TODO: Specifying the port on every single command is a bit cumbersome.
//       Maybe choose a default port that gets set automatically????

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Start server process")]
  Serve {
    #[arg(short, long)]
    port: u32,

    #[arg(short, long)]
    config_path: String,
  },

  #[command(about = "Show current commands")]
  Show {
    #[arg(short, long)]
    port: u32,
  },

  #[command(about = "Edit commands")]
  Edit {
    #[arg(short, long)]
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

// TODO: I think the modifications should be done here.
//       Make all subcommands return a Result<String>. It should have contain either the text or the error message
//       then it should return with code=1 here in this place.
fn match_cli_command(cli: &Cli) {
  if let Some(command) = &cli.command {
    let result = match command {
      Commands::Serve { port, config_path } => {
        server::main::start(*port, config_path);
        Ok(String::new())
      }
      Commands::Edit { port } => edit_subcommand(*port),
      Commands::Show { port } => api_client::get_current_config(*port),
    };

    match result {
      Ok(msg) => {
        println!("{msg}");
      }
      Err(err) => {
        // TODO: Temp color print. Find a better way??
        //       Mind that only the CLI tool needs to have colors, not the server.
        //       The message "Failed to install commands: Some commands have incorrect format" is also
        //       printed in the server-side, as well as the frontend, but only colorize the CLI, not the
        //       the server (reading colored logs is hideous).
        eprintln!("\x1b[93m{err}\x1b[0m");
        // eprintln!("{err}");
        std::process::exit(1);
      }
    }
  }
}

pub fn start_cli_app() {
  let cli = Cli::parse();
  match_cli_command(&cli);
}
