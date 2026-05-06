use std::io::Write;

use env_logger::Env;
use log::{LevelFilter, debug, info};

pub mod cli;
pub mod commands;

fn main() {
    let (command, config) = cli::to_config();

    if command.is_none() {
        cli::print_help();
        std::process::exit(0);
    };

    let command = command.unwrap();
    let log_level: &str;

    match config.verbose {
        0 => {
            log_level = LevelFilter::Error.as_str();
        },

        1 => {
            log_level = LevelFilter::Warn.as_str();
        },

        2 => {
            log_level = LevelFilter::Info.as_str();
        },

        _ => {
            log_level = LevelFilter::Debug.as_str();
        },
    }

    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();

    debug!(target: "cli::main", "log level: `{}`", format!("{}", log_level).to_lowercase());
    info!(target: "cli::main", "running command `{}`", format!("{:?}", &command).to_lowercase());

    if let Err(err) = cli::run_command(command, config) {
        log::error!("application error: `{err}`");
    }

    debug!(target: "cli::main", "exiting with return code `0`");

    std::io::stderr().flush().ok();
    std::io::stdout().flush().ok();
}
