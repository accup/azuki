use std::io::stdin;

use azuki::core::{
    converter::Converter,
    packed_bits::{PackedBitsCompressor, PackedBitsExtractor},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim_end();
    let size: usize = input.parse().unwrap();

    let mut packed = Vec::new();
    let mut packed_bits = PackedBitsCompressor::new(&mut packed);
    packed_bits
        .convert(&(0..size).map(|v| v as u8).collect::<Vec<_>>())
        .unwrap();

    println!(
        "({:x} + {:x})\n({:x}) {:x?}",
        size,
        packed.len() - size,
        packed.len(),
        &packed[..packed.len().min(20)]
    );

    let mut unpacked = Vec::new();
    let mut packed_bits = PackedBitsExtractor::new(&mut unpacked);
    packed_bits.convert(&packed).unwrap();

    println!(
        "({:x}) {:x?}",
        unpacked.len(),
        &unpacked[..unpacked.len().min(20)]
    );
}
