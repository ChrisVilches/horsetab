use super::{
  global_context::MainProcessState,
  global_context_installer::{install_state_from_file, InstallResult},
  process_manager::ProcessManager,
};
use crate::{cmd::Cmd, sequence_automata::AutomataInstruction, util::read_lines_or_create};
use anyhow::{bail, Result};
use rouille::{Request, Response, Server};
use std::{
  error::Error,
  fs::OpenOptions,
  io::{Read, Write},
  sync::{mpsc::Sender, Arc, Mutex},
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
  let content = read_lines_or_create(config_path)?.join("\n");
  Ok(Response::text(content))
}

fn reinstall(
  request: &Request,
  config_path: &str,
  state: &mut MainProcessState,
) -> Result<Response> {
  let new_content = get_body_as_string(request)?;
  update_config_file(config_path, &new_content)?;
  let install_result = install_state_from_file(config_path, state);

  if let InstallResult::FileError(err) = install_result {
    bail!(err);
  }

  Ok(Response::text(install_result.to_string()).with_status_code(200))
}

#[allow(clippy::unnecessary_wraps)]
fn curr_cmds(commands: &[Cmd]) -> Result<Response> {
  let current_commands_text = commands
    .iter()
    .map(|cmd| format!("{} {}", cmd.sequence, cmd.command))
    .collect::<Vec<String>>()
    .join("\n");

  Ok(Response::text(current_commands_text))
}

fn send_sequence(
  request: &Request,
  sequence_sender: &Sender<AutomataInstruction>,
) -> Result<Response> {
  let seq = get_body_as_string(request)?;
  sequence_sender.send(AutomataInstruction::Reset)?;

  for c in seq.chars() {
    sequence_sender.send(AutomataInstruction::Char(c))?;
  }
  sequence_sender.send(AutomataInstruction::Reset)?;

  Ok(Response::empty_204())
}

#[allow(clippy::unnecessary_wraps)]
fn get_ps(process_manager: &ProcessManager) -> Result<Response> {
  Ok(Response::text(process_manager.format_information()))
}

#[allow(clippy::unnecessary_wraps)]
fn get_tcp_port(tcp_port: u16) -> Result<Response> {
  Ok(Response::text(format!("{tcp_port}")))
}

fn build_http_server(
  port: u16,
  tcp_port: u16,
  config_path: &str,
  sequence_sender: Sender<AutomataInstruction>,
  state: Arc<Mutex<MainProcessState>>,
) -> Result<Server<impl Fn(&Request) -> Response>, Box<dyn Error + Send + Sync>> {
  let conf_path = config_path.to_owned();

  Server::new(format!("0.0.0.0:{port}"), move |req| {
    handle_response(match (req.method(), req.url().as_ref()) {
      ("GET", "/current-config-file-content") => read_config_file(&conf_path),
      ("GET", "/ps") => get_ps(&state.lock().unwrap().process_manager),
      ("GET", "/tcp-port") => get_tcp_port(tcp_port),
      ("GET", "/current-installed-commands") => curr_cmds(&state.lock().unwrap().commands),
      ("POST", "/send-sequence") => send_sequence(req, &sequence_sender),
      ("PUT", "/re-install") => reinstall(req, &conf_path, &mut state.lock().unwrap()),
      _ => Ok(Response::text("Not found").with_status_code(404)),
    })
  })
}

pub fn start_http_server(
  port: u16,
  tcp_port: u16,
  config_path: &str,
  sequence_sender: Sender<AutomataInstruction>,
  state: Arc<Mutex<MainProcessState>>,
) {
  match build_http_server(port, tcp_port, config_path, sequence_sender, state) {
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
