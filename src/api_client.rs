use anyhow::Result;
use reqwest::StatusCode;

fn build_url(port: u16, path: &str) -> String {
  format!("http://localhost:{port}/{path}")
}

pub fn reinstall_commands(port: u16, new_content: &str) -> Result<String> {
  let client = reqwest::blocking::Client::new();
  let res = client
    .put(build_url(port, "re-install"))
    .body(new_content.to_owned())
    .send()?;

  match res.status() {
    StatusCode::OK => Ok(res.text()?),
    _ => Err(anyhow::anyhow!("{}", res.text()?)),
  }
}

pub fn get_ps(port: u16) -> Result<String> {
  Ok(
    reqwest::blocking::get(build_url(port, "ps"))?
      .error_for_status()?
      .text()?,
  )
}

pub fn get_current_config(port: u16) -> Result<String> {
  let res =
    reqwest::blocking::get(build_url(port, "current-config-file-content"))?.error_for_status()?;
  Ok(res.text()?)
}

pub fn get_current_installed_commands(port: u16) -> Result<String> {
  Ok(
    reqwest::blocking::get(build_url(port, "current-installed-commands"))?
      .error_for_status()?
      .text()?,
  )
}

pub fn get_tcp_port(port: u16) -> Result<u16> {
  let res = reqwest::blocking::get(build_url(port, "tcp-port"))?.error_for_status()?;
  let text = res.text()?;
  Ok(str::parse(&text)?)
}

pub fn send_sequence(port: u16, sequence: &str) -> Result<String> {
  let client = reqwest::blocking::Client::new();
  let res = client
    .post(build_url(port, "send-sequence"))
    .body(sequence.to_owned())
    .send()?;

  match res.status() {
    StatusCode::OK => Ok(res.text()?),
    StatusCode::NO_CONTENT => Ok(String::new()),
    _ => Err(anyhow::anyhow!("{}", res.text()?)),
  }
}
