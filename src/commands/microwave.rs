use clap::Args;

use azuki::core::lz77::LZ77;

use crate::commands::{
    io::{Reading, Writing},
    Command,
};

use super::io::{with_extension, without_extension};

pub struct MicrowaveCommand;

#[derive(Args)]
pub struct MicrowaveCommandArgs {
    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,
}

impl Command for MicrowaveCommand {
    type Args = MicrowaveCommandArgs;

    fn execute(&self, args: &Self::Args) -> std::io::Result<()> {
        let input_path = args.input.clone();
        let output_path = args.output.clone();
        let output_path = output_path.or(with_extension(
            without_extension(input_path.as_deref(), "frozen").as_deref(),
            "microwaved",
        ));

        let mut reading = Reading::open(input_path.as_deref())?;
        let mut writing = Writing::create(output_path.as_deref())?;

        LZ77::extract(&reading.read_data()?, &mut writing)?;

        Ok(())
    }
}
