use std::io::stdin;

use azuki::core::packed_bits::PackedBits;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim_end();
    let size: usize = input.parse().unwrap();

    let data = (0..size).map(|v| v as u8).collect::<Vec<_>>();

    let mut packed = vec![Default::default(); PackedBits::measure(&data)];
    PackedBits::compress(&data, &mut packed);

    println!(
        "({:x} + {:x})\n({:x}) {:x?}",
        size,
        packed.len() - size,
        packed.len(),
        &packed[..packed.len().min(20)]
    );

    let mut unpacked = PackedBits::prepare(&packed);
    PackedBits::extract(&packed, &mut unpacked);

    println!(
        "({:x}) {:x?}",
        unpacked.len(),
        &unpacked[..unpacked.len().min(20)]
    );
}
