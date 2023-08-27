use clap::Args;

use azuki::core::lz77::LZ77;

use crate::commands::{
    io::{with_extension, Reading, Writing},
    Command,
};

pub struct FreezeCommand;

#[derive(Args)]
pub struct FreezeCommandArgs {
    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,
}

impl Command for FreezeCommand {
    type Args = FreezeCommandArgs;

    fn execute(&self, args: &Self::Args) -> std::io::Result<()> {
        let input_path = args.input.clone();
        let output_path = args.output.clone();
        let output_path = output_path.or(with_extension(input_path.as_deref(), "frozen"));

        let mut reading = Reading::open(input_path.as_deref())?;
        let mut writing = Writing::create(output_path.as_deref())?;

        LZ77::compress(&reading.read_data()?, &mut writing)?;

        Ok(())
    }
}
