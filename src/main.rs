mod cli;
mod commands;
mod model;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            commands::start::run();
        }
        Commands::Log => {
            commands::log::run();
        }
        Commands::Break => {
            commands::rest::run();
        }
    }
}

