mod cli;
mod commands;
mod model;
mod storage;
mod config;
mod timer;
mod signal;


use clap::Parser;
use cli::{Cli, Commands, ConfigAction};

fn main() {
    signal::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start(args) => {
           commands::start::start(
                args.auto,
                args.cycles.unwrap_or_else(|| config::load().cycles),
            );
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
            ConfigAction::Cycles { cycles } => commands::config::set_cycles(cycles),
            ConfigAction::Show => commands::config::show(),
        },
    }
}

