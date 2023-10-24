use super::commands_installer::{install_commands, InstallResult};
use crate::{cmd_parser::Cmd, sequence_automata::SequenceAutomata};
use anyhow::Result;
use rouille::{Request, Response, Server};
use std::{
  error::Error,
  fs::{self, OpenOptions},
  io::{Read, Write},
  sync::{Arc, Mutex},
};

fn handle_response(response: Result<Response>) -> Response {
  match response {
    Ok(res) => res,
    Err(err) => Response::text(format!("Error: {err}")).with_status_code(500),
  }
}

fn update_config_file(config_path: &str, new_content: &str) -> Result<()> {
  let mut file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(config_path)?;

  file.write_all(new_content.as_bytes())?;

  Ok(())
}

fn get_body_as_string(request: &Request) -> Result<String> {
  match request.data() {
    Some(mut request_body) => {
      let mut buf = String::new();
      request_body.read_to_string(&mut buf)?;
      Ok(buf)
    }
    None => Ok(String::new()),
  }
}

fn read_config_file(config_path: &str) -> Result<Response> {
  Ok(Response::text(fs::read_to_string(config_path)?))
}

fn reinstall_commands(
  request: &Request,
  config_path: &str,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
) -> Result<Response> {
  let new_content = get_body_as_string(request)?;
  update_config_file(config_path, &new_content)?;
  let install_result = install_commands(config_path, automata, commands);

  let status_code = match install_result {
    InstallResult::Error(_) => 400,
    _ => 200,
  };

  Ok(Response::text(install_result.to_string()).with_status_code(status_code))
}

fn build_http_server(
  port: u32,
  config_path: &str,
  automata: Arc<Mutex<SequenceAutomata>>,
  commands: Arc<Mutex<Vec<Cmd>>>,
) -> Result<Server<impl Fn(&Request) -> Response>, Box<dyn Error + Send + Sync>> {
  let config_path_clone = config_path.to_owned();

  Server::new(format!("0.0.0.0:{port}"), move |request| {
    let method = request.method();
    let url = request.url();

    let response = match (method, url.as_ref()) {
      ("GET", "/current-config-file-content") => read_config_file(&config_path_clone),
      ("PUT", "/re-install") => {
        reinstall_commands(request, &config_path_clone, &automata, &commands)
      }
      _ => Ok(Response::text("Not found").with_status_code(404)),
    };

    handle_response(response)
  })
}

pub fn start_http_server(
  port: u32,
  config_path: &str,
  automata: Arc<Mutex<SequenceAutomata>>,
  commands: Arc<Mutex<Vec<Cmd>>>,
) {
  match build_http_server(port, config_path, automata, commands) {
    Ok(server) => {
      println!("Listening on {:?}", server.server_addr());
      server.run();
    }
    Err(err) => {
      eprintln!("Cannot start server");
      eprintln!("{err}");
      std::process::exit(1);
    }
  }
}
