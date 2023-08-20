pub mod commands;

use crate::commands::{
    Command, FreezeCommand, FreezeCommandArgs, MicrowaveCommand, MicrowaveCommandArgs,
};

use clap::{Parser, Subcommand};

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
