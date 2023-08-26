use std::path::Path;

use clap::Args;

use azuki::core::dump;

use super::command::Command;

pub struct DumpCommand;

#[derive(Args)]
pub struct DumpCommandArgs {
    pub filename: String,
}

impl Command for DumpCommand {
    type Args = DumpCommandArgs;

    fn execute(&self, args: &Self::Args) {
        let input_path = Path::new(&args.filename);
        let output_path_buf = {
            let mut buf = input_path.to_path_buf();

            let dump_ext = String::from("dump");
            let ext = buf
                .extension()
                .map(|ext| {
                    ext.to_str()
                        .map_or(dump_ext.clone(), |ext| format!("{}.{}", ext, dump_ext))
                })
                .unwrap_or(dump_ext.clone());
            buf.set_extension(ext);

            buf
        };
        let output_path = output_path_buf.as_path();

        if let Err(e) = dump(input_path, output_path) {
            eprintln!("ERROR: {}", e);
            return;
        }

        println!("{} ...> {}", input_path.display(), output_path.display());
    }
}
