pub mod commands;

use crate::commands::{
    Command, FreezeCommand, FreezeCommandArgs, MicrowaveCommand, MicrowaveCommandArgs,
};

use clap::{Parser, Subcommand};
use commands::{DumpCommand, DumpCommandArgs};

#[derive(Parser)]
#[command(name = "azuki", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "f")]
    #[command(alias = "fr")]
    #[command(alias = "fre")]
    #[command(alias = "free")]
    #[command(alias = "freez")]
    Freeze(FreezeCommandArgs),

    #[command(alias = "m")]
    #[command(alias = "mi")]
    #[command(alias = "mic")]
    #[command(alias = "micr")]
    #[command(alias = "micro")]
    #[command(alias = "microw")]
    #[command(alias = "microwa")]
    #[command(alias = "microwav")]
    Microwave(MicrowaveCommandArgs),

    #[command(alias = "d")]
    #[command(alias = "du")]
    #[command(alias = "dum")]
    Dump(DumpCommandArgs),
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Freeze(args) => FreezeCommand.execute(args)?,
        Commands::Microwave(args) => MicrowaveCommand.execute(args)?,
        Commands::Dump(args) => DumpCommand.execute(args)?,
    }

    Ok(())
}
