use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "azuki", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Freeze {},
    Microwave {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Freeze {} => {
            println!("バッ、ゴトッ、バン")
        }

        Commands::Microwave {} => {
            println!("バッ、スッ、バン、ガチャッ、ゴトッ、ガン、ピッ、ピッ、ジー、ピー、ピー、ガチャッ、スッ、ガン、アチチチッ")
        }
    }
}
