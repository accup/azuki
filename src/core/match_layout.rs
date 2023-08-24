use std::io::Write;

use super::{
    converter::Converter,
    head::{Head, LeadingZero},
};

pub struct MatchLayoutCompressor<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> MatchLayoutCompressor<'a, W> {
    fn compress(left: usize, count: usize, buffer: &mut [u8]) {
        Head::<LeadingZero>::compress(left, buffer);
        Head::<LeadingZero>::compress(count, buffer);
    }

    pub fn measure(left: usize, count: usize) -> usize {
        Head::<LeadingZero>::measure(left) + Head::<LeadingZero>::measure(count)
    }
}

impl<'a, W: Write> MatchLayoutCompressor<'a, W> {
    fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    fn convert(&mut self, left: usize, count: usize) -> std::io::Result<()> {
        let size = Self::measure(left, count);
        let mut buffer = vec![0; size];
        Self::compress(left, count, &mut buffer);
        self.writer.write_all(&buffer)?;
        Ok(())
    }
}

pub struct Match {
    pub left: usize,
    pub count: usize,
}

pub trait MatchLayout {
    const MAX_LEFT: usize;
    const MAX_COUNT: usize;

    fn check(buffer: &[u8; 2]) -> bool;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<usize>;

    fn read(buffer: &[u8; 2]) -> Match;
}

pub struct MatchLayoutC2L13;

impl MatchLayout for MatchLayoutC2L13 {
    const MAX_LEFT: usize = 8192;
    const MAX_COUNT: usize = 4;

    fn check(buffer: &[u8; 2]) -> bool {
        return (buffer[0] & 0x80u8) != 0;
    }

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<usize> {
        // 1CCLLLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = ((left - 1) as u64).to_le_bytes();
        let le_count = ((count - 1) as u64).to_le_bytes();
        let buffer = [0x80u8 | (le_count[0] << 5) | le_left[1], le_left[0]];
        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }

    fn read(buffer: &[u8; 2]) -> Match {
        let left =
            (u64::from_le_bytes([buffer[1], buffer[0] & 0x1Fu8, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        let count =
            (u64::from_le_bytes([(buffer[0] >> 5) & 0x03u8, 0, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        Match { left, count }
    }
}

pub struct MatchLayoutC3L12;

impl MatchLayout for MatchLayoutC3L12 {
    const MAX_LEFT: usize = 4096;
    const MAX_COUNT: usize = 8;

    fn check(buffer: &[u8; 2]) -> bool {
        return (buffer[0] & 0x80u8) != 0;
    }

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<usize> {
        // 1CCCLLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = ((left - 1) as u64).to_le_bytes();
        let le_count = ((count - 1) as u64).to_le_bytes();
        let buffer = [0x80u8 | (le_count[0] << 4) | le_left[1], le_left[0]];
        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }

    fn read(buffer: &[u8; 2]) -> Match {
        let left =
            (u64::from_le_bytes([buffer[1], buffer[0] & 0x0Fu8, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        let count =
            (u64::from_le_bytes([(buffer[0] >> 4) & 0x07u8, 0, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        Match { left, count }
    }
}

pub struct MatchLayoutC4L11;

impl MatchLayout for MatchLayoutC4L11 {
    const MAX_LEFT: usize = 2048;
    const MAX_COUNT: usize = 16;

    fn check(buffer: &[u8; 2]) -> bool {
        return (buffer[0] & 0x80u8) != 0;
    }

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<usize> {
        // 1CCCCLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = ((left - 1) as u64).to_le_bytes();
        let le_count = ((count - 1) as u64).to_le_bytes();
        let buffer = [0x80u8 | (le_count[0] << 3) | le_left[1], le_left[0]];
        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }

    fn read(buffer: &[u8; 2]) -> Match {
        let left =
            (u64::from_le_bytes([buffer[1], buffer[0] & 0x07u8, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        let count =
            (u64::from_le_bytes([(buffer[0] >> 3) & 0x0Fu8, 0, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        Match { left, count }
    }
}

pub struct MatchLayoutL7C8;

impl MatchLayout for MatchLayoutL7C8 {
    const MAX_LEFT: usize = 128;
    const MAX_COUNT: usize = 256;

    fn check(buffer: &[u8; 2]) -> bool {
        return (buffer[0] & 0x80u8) != 0;
    }

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<usize> {
        // 1LLLLLLL CCCCCCCC (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = ((left - 1) as u64).to_le_bytes();
        let le_count = ((count - 1) as u64).to_le_bytes();
        let buffer = [0x80u8 | le_count[0], le_left[0]];
        writer.write_all(&buffer)?;

        Ok(buffer.len())
    }

    fn read(buffer: &[u8; 2]) -> Match {
        let left = (u64::from_le_bytes([buffer[0] & 0x7Fu8, 0, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        let count = (u64::from_le_bytes([buffer[1], 0, 0, 0, 0, 0, 0, 0]) as usize) + 1;
        Match { left, count }
    }
}
