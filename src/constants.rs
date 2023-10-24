use home::home_dir;

pub static DEFAULT_PORT: u32 = 17757;
static DEFAULT_CONFIG_FILE_NAME: &str = ".horsetab.conf";

pub fn get_default_config_path() -> String {
  home_dir()
    .map(|p| p.join(DEFAULT_CONFIG_FILE_NAME))
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default()
    .to_owned()
}
