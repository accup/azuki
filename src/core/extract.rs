use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use super::{bar::new_bar, lz77::LZ77Extract};

pub fn extract(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let original_size = input_file.metadata().map_or(0, |x| x.len());
    let pb = new_bar(original_size);

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let mut lz77 = LZ77Extract::new();

    while lz77.extract_next(&mut reader, &mut writer)? {
        let length = (original_size as usize) - lz77.bytes_read() + lz77.bytes_written();
        let position = lz77.bytes_written();

        pb.set_length(length as u64);
        pb.set_position(position as u64)
    }

    Ok(())
}

pub trait Extract {
    fn extract_next(
        &mut self,
        reader: &mut impl Read,
        writer: &mut impl Write,
    ) -> std::io::Result<bool>;
}
