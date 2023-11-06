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
#![allow(clippy::significant_drop_tightening)]

mod api_client;
mod cli;
mod click_sequence_detector;
mod cmd;
mod constants;
mod ipc_tcp;
mod logger;
mod sequence_automata;
mod server;
mod util;

fn main() {
  cli::main::start_cli_app();
}
