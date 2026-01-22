use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "pomodoro",
    version,
    about = "CLI to control focus time like Pomodoro"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Log,
    Break,
    Stats,
}


