#![allow(unused, clippy::field_reassign_with_default)]

use playback_logger::{Level, debug, error, info};
use std::io::Write;

pub mod cli;
pub mod commands;

fn main() {
    let (command, config) = cli::to_config();

    if command.is_none() {
        cli::print_help();

        std::process::exit(0);
    }

    let command = command.unwrap();

    let log_level = match config.verbose {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        _ => Level::Debug,
    };

    playback_logger::init(log_level);

    debug!("cli::main", "log level: `{}`", log_level.as_str().to_lowercase());
    info!("cli::main", "running command `{}`", format!("{:?}", &command).to_lowercase());

    if let Err(err) = cli::run_command(command, config) {
        error!("cli::main", "application error: {err}");
    }

    debug!("cli::main", "exiting with return code `0`");

    std::io::stderr().flush().ok();
    std::io::stdout().flush().ok();
}
