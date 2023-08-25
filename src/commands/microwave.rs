use std::path::Path;

use clap::Args;

use azuki::core::extract;

use super::command::Command;

pub struct MicrowaveCommand;

#[derive(Args)]
pub struct MicrowaveCommandArgs {
    pub filename: String,
}

impl Command for MicrowaveCommand {
    type Args = MicrowaveCommandArgs;

    fn execute(&self, args: &Self::Args) {
        let input_path = Path::new(&args.filename);
        let output_path_buf = {
            let mut buf = input_path.to_path_buf();
            let ext = buf
                .extension()
                .map(|ext| ext.to_str().unwrap_or(""))
                .unwrap_or("");
            if ext == "frozen" {
                buf.set_extension("");
            }

            let ext = buf
                .extension()
                .map(|ext| ext.to_str().unwrap_or(""))
                .unwrap_or("");

            buf.set_extension(if ext.is_empty() {
                String::from("hot")
            } else {
                format!("hot.{}", ext)
            });

            buf
        };
        let output_path = output_path_buf.as_path();

        if let Err(e) = extract(input_path, output_path) {
            eprintln!("ERROR: {}", e);
            return;
        }

        println!("{} ...> {}", input_path.display(), output_path.display());
    }
}
