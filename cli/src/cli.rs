#![allow(unused)]

use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use clap::{ArgAction, CommandFactory, Parser, Subcommand, ValueEnum};
use libplayback::config::Config;

use crate::commands;

#[derive(Parser, Debug)]
#[command(name = "libplayback", version = "0.1.0", about = "a simple workflow manager")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "whether or not to enable all logging",
        action = ArgAction::Count
    )]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    #[command(name = "new", about = "create a new libplayback workflow")]
    New { name: String },

    #[command(name = "run", about = "replay a libplayback workflow")]
    Run {
        name: String,

        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

pub fn run_command(command: Subcommands, config: Config) -> Result<(), Box<dyn Error>> {
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
    let args = Cli::parse();
    let mut config = Config::default();

    config.verbose = args.verbose;

    (args.command, config)
}

pub fn print_help() {
    Cli::command().print_help().unwrap();

    println!();
}
