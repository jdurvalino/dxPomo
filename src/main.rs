mod cli;
mod commands;
mod model;
mod storage;
mod config;

use clap::Parser;
use cli::{Cli, Commands, ConfigAction};

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
        Commands::Stats => {
            commands::stats::run();
        }
        Commands::Config { action } => match action {
            ConfigAction::Focus { minutes } => commands::config::set_focus(minutes),
            ConfigAction::Break { minutes } => commands::config::set_break(minutes),
            ConfigAction::Show => commands::config::show(),
        },
    }
}

