use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use super::lz77::LZ77Compress;

pub fn compress(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let mut lz77 = LZ77Compress::new();

    while lz77.next(&mut reader, &mut writer)? {}

    Ok(())
}
