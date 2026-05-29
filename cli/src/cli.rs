#![allow(unused)]

use crate::commands;
use miniclap::{App, Arg, ArgKind};
use playback_lib::config::Config;
use std::error::Error;

#[derive(Debug)]
pub enum Subcommands {
    New { name: String },
    Run { name: String, args: Vec<String> },
}

pub fn build_cli() -> App {
    App::new("playback")
        .about("a simple workflow manager")
        .arg(Arg::new("verbose", 'v', ArgKind::Count))
        .subcommand(App::new("new").about("create a new playback workflow").arg(Arg::positional("name")))
        .subcommand(App::new("run").about("replay a playback workflow").arg(Arg::positional("name")).arg(Arg::positional("args")))
}

pub fn run_command(command: Subcommands, _config: Config) -> Result<(), Box<dyn Error>> {
    match command {
        Subcommands::New { name } => {
            commands::new(name.as_str())?;
        }

        Subcommands::Run { name, args } => {
            commands::run(name.as_str(), args)?;
        }
    }

    Ok(())
}

pub fn to_config() -> (Option<Subcommands>, Config) {
    let argv: Vec<String> = std::env::args().skip(1).collect();

    let app = build_cli();
    let matches = app.parse();

    let mut config = Config {
        verbose: matches.count("verbose") as u8,
    };

    config.verbose = matches.count("verbose") as u8;

    if let Some(sub) = matches.subcommand("new") {
        let positionals = sub.positionals();
        let name = positionals.first().cloned().unwrap_or_default();

        return (Some(Subcommands::New { name }), config);
    }

    if let Some(sub) = matches.subcommand("run") {
        let positionals = sub.positionals();
        let name = positionals.first().cloned().unwrap_or_default();

        let args = if positionals.len() > 1 { positionals[1..].to_vec() } else { vec![] };

        return (Some(Subcommands::Run { name, args }), config);
    }

    if let Some((index, workflow_name)) = argv.iter().enumerate().find(|(_, arg)| !arg.starts_with('-')) {
        let workflow_path = format!(".playback/workflows/{}.toml", workflow_name);

        if std::path::Path::new(&workflow_path).exists() {
            let args = if argv.len() > index + 1 { argv[index + 1..].to_vec() } else { vec![] };

            return (Some(Subcommands::Run { name: workflow_name.clone(), args }), config);
        }
    }

    (None, config)
}

pub fn print_help() {
    build_cli().print_help();
}
