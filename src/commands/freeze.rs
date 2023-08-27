use clap::Args;

use azuki::core::{
    bwt::bwt,
    lz77::LZ77,
    suffix_array::{suffix_array, U8Bucket},
};

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

    #[arg(long)]
    pub bwt: bool,
}

impl Command for FreezeCommand {
    type Args = FreezeCommandArgs;

    fn execute(&self, args: &Self::Args) -> std::io::Result<()> {
        let input_path = args.input.clone();
        let output_path = args.output.clone();
        let output_path = output_path.or({
            let mut path = input_path.clone();
            if args.bwt {
                path = with_extension(path.as_deref(), "bwt");
            }
            path = with_extension(path.as_deref(), "frozen");
            path
        });

        let mut reading = Reading::open(input_path.as_deref())?;
        let mut writing = Writing::create(output_path.as_deref())?;

        let mut data = reading.read_data()?;

        if args.bwt {
            let suffix_array = suffix_array(&data, &U8Bucket);
            data = bwt(&data, &suffix_array);
        }

        LZ77::compress(&data, &mut writing)?;

        Ok(())
    }
}
