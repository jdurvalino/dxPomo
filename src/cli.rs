use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dxPomo",
    version,
    about = "CLI to control focus time like Pomodoro"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start(StartArgs),
    Log,
    Break,
    Stats,
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    Focus { minutes: u64 },
    Break { minutes: u64 },
    Show,
}

#[derive(clap::Parser)]
pub struct StartArgs {
    #[arg(long)]
    pub auto: bool, 

    #[arg(long)]
    pub cycles: Option<u32>,

}

