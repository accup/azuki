use crate::core::Command;

use clap::{Parser, Subcommand};
use commands::{FreezeCommand, FreezeCommandArgs};

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
    Microwave {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Freeze(args) => FreezeCommand.execute(args),

        Commands::Microwave {} => {
            println!("バッ、スッ、バン、ガチャッ、ゴトッ、ガン、ピッ、ピッ、ジー、ピー、ピー、ガチャッ、スッ、ガン、アチチチッ")
        }
    }
}
