use super::{
  commands_installer::{install_commands, read_lines_or_create, InstallResult},
  event_observe::EventSubscriber,
};
use crate::{
  cmd::Cmd,
  sequence_automata::{AutomataInstruction, SequenceAutomata},
};
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

fn reinstall_config(
  request: &Request,
  config_path: &str,
  automata: &Mutex<SequenceAutomata>,
  commands: &Mutex<Vec<Cmd>>,
  shell_script: &Mutex<String>,
  interpreter: &Mutex<Vec<String>>,
) -> Result<Response> {
  let new_content = get_body_as_string(request)?;
  update_config_file(config_path, &new_content)?;
  let install_result = install_commands(config_path, automata, commands, shell_script, interpreter);

  if let InstallResult::FileError(err) = install_result {
    bail!(err);
  }

  Ok(Response::text(install_result.to_string()).with_status_code(200))
}

#[allow(clippy::unnecessary_wraps)]
fn get_current_installed_commands(commands: &Mutex<Vec<Cmd>>) -> Result<Response> {
  let current_commands_text = commands
    .lock()
    .unwrap()
    .iter()
    .map(|cmd| format!("{} {}", cmd.sequence, cmd.command))
    .collect::<Vec<String>>()
    .join("\n");

  Ok(Response::text(current_commands_text))
}

fn watch_sequences(
  request: &Request,
  event_subscriber: &Mutex<EventSubscriber>,
) -> Result<Response> {
  let file = get_body_as_string(request)?;
  event_subscriber.lock().unwrap().subscribe(&file);
  Ok(Response::empty_204())
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

#[allow(clippy::too_many_lines)]
fn build_http_server(
  port: u32,
  config_path: &str,
  sequence_sender: Sender<AutomataInstruction>,
  event_subscriber: Arc<Mutex<EventSubscriber>>,
  automata: Arc<Mutex<SequenceAutomata>>,
  commands: Arc<Mutex<Vec<Cmd>>>,
  shell_script: Arc<Mutex<String>>,
  interpreter: Arc<Mutex<Vec<String>>>,
) -> Result<Server<impl Fn(&Request) -> Response>, Box<dyn Error + Send + Sync>> {
  let config_path_clone = config_path.to_owned();

  Server::new(format!("0.0.0.0:{port}"), move |request| {
    let method = request.method();
    let url = request.url();

    let response = match (method, url.as_ref()) {
      ("GET", "/current-config-file-content") => read_config_file(&config_path_clone),
      ("GET", "/current-installed-commands") => get_current_installed_commands(&commands),
      ("POST", "/observe-sequences") => watch_sequences(request, &event_subscriber),
      ("POST", "/send-sequence") => send_sequence(request, &sequence_sender),
      ("PUT", "/re-install") => reinstall_config(
        request,
        &config_path_clone,
        &automata,
        &commands,
        &shell_script,
        &interpreter,
      ),
      _ => Ok(Response::text("Not found").with_status_code(404)),
    };

    handle_response(response)
  })
}

#[allow(clippy::too_many_lines)]
pub fn start_http_server(
  port: u32,
  config_path: &str,
  sequence_sender: Sender<AutomataInstruction>,
  event_subscriber: Arc<Mutex<EventSubscriber>>,
  automata: Arc<Mutex<SequenceAutomata>>,
  commands: Arc<Mutex<Vec<Cmd>>>,
  shell_script: Arc<Mutex<String>>,
  interpreter: Arc<Mutex<Vec<String>>>,
) {
  match build_http_server(
    port,
    config_path,
    sequence_sender,
    event_subscriber,
    automata,
    commands,
    shell_script,
    interpreter,
  ) {
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
