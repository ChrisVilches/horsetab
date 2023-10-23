use super::commands_installer::{install_commands, InstallResult};
use crate::{cmd_parser::Cmd, sequence_automata::SequenceAutomata};
use rouille::Response;
use std::sync::{Arc, Mutex};

pub fn start_http_server(
  port: &str,
  config_path: &String,
  automata: &Arc<Mutex<SequenceAutomata>>,
  commands: &Arc<Mutex<Vec<Cmd>>>,
) {
  // TODO: Too much cloning...
  // TODO: parameters should be OK without Arc<T> I think.
  let automata_clone = Arc::clone(&automata);
  let commands_clone = Arc::clone(&commands);
  let config_path_clone = config_path.clone();

  rouille::start_server(format!("0.0.0.0:{port}"), move |request| {
    let method = request.method();
    let url = request.url();

    match (method, url.as_ref()) {
      ("GET", "/config-path") => Response::text(&config_path_clone).with_status_code(200),
      ("PUT", "/re-install") => {
        let install_result = install_commands(&config_path_clone, &automata_clone, &commands_clone);

        let status_code = match install_result {
          InstallResult::Error(_) => 400,
          _ => 200,
        };

        Response::text(install_result.to_string()).with_status_code(status_code)
      }
      _ => Response::text("Not found").with_status_code(404),
    }
  });
}
