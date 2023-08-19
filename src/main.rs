use crate::core::Command;

use clap::{Parser, Subcommand};
use commands::{FreezeCommand, FreezeCommandArgs, MicrowaveCommand, MicrowaveCommandArgs};

pub mod commands;
pub mod core;

#[derive(Parser)]
#[command(name = "azuki", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Freeze(FreezeCommandArgs),
    Microwave(MicrowaveCommandArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Freeze(args) => FreezeCommand.execute(args),
        Commands::Microwave(args) => MicrowaveCommand.execute(args),
    }
}
