use std::path::Path;

use clap::Args;

use crate::core::{compress, Command};

pub struct FreezeCommand;

#[derive(Args)]
pub struct FreezeCommandArgs {
    pub filename: String,
}

impl Command for FreezeCommand {
    type Args = FreezeCommandArgs;

    fn execute(&self, args: &Self::Args) {
        let input_path = Path::new(&args.filename);
        let output_path_buf = {
            let mut buf = input_path.to_path_buf();
            let ext = buf
                .extension()
                .map(|ext| ext.to_str().unwrap_or(""))
                .unwrap_or("");
            buf.set_extension(String::from(ext) + ".frozen");
            buf
        };
        let output_path = output_path_buf.as_path();

        println!("{} ...> {}", input_path.display(), output_path.display());

        if let Err(e) = compress(input_path, output_path) {
            eprintln!("ERROR: {}", e);
        }
    }
}
