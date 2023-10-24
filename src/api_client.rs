use anyhow::Result;
use reqwest::StatusCode;

fn build_url(port: u32, path: &str) -> String {
  format!("http://localhost:{port}/{path}")
}

pub fn reinstall_commands(port: u32, new_content: &str) -> Result<String> {
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

pub fn get_current_config(port: u32) -> Result<String> {
  let res =
    reqwest::blocking::get(build_url(port, "current-config-file-content"))?.error_for_status()?;
  Ok(res.text()?)
}
