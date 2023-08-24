use std::{io::Write, mem::size_of};

use super::converter::Converter;

pub struct PackedBitsCompressor<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> PackedBitsCompressor<'a, W> {
    fn compress(data: &[u8], buffer: &mut [u8]) {
        let head_size = Self::compress_head(data.len(), buffer);
        buffer[head_size..(head_size + data.len())].copy_from_slice(data);
    }

    fn compress_head(byte_count: usize, buffer: &mut [u8]) -> usize {
        let head_size = Self::measure_head_size(byte_count);

        let bytes = byte_count.to_be_bytes();
        let bytes = &bytes[(size_of::<usize>().saturating_sub(head_size))..];

        buffer[(head_size - bytes.len())..head_size].copy_from_slice(bytes);
        buffer[0] |= match byte_count {
            0x0000000000000000..=0x000000000000003F => 0x80,
            0x0000000000000040..=0x0000000000001FFF => 0xC0,
            0x0000000000002000..=0x00000000000FFFFF => 0xE0,
            0x0000000000100000..=0x0000000007FFFFFF => 0xF0,
            0x0000000008000000..=0x00000003FFFFFFFF => 0xF8,
            0x0000000400000000..=0x000001FFFFFFFFFF => 0xFC,
            0x0000020008000000..=0x0000FFFFFFFFFFFF => 0xFE,
            0x0001000000000000..=0xFFFFFFFFFFFFFFFF => 0xFF,
            _ => panic!("Too high number {:x}", 0),
        };

        if head_size > 1 {
            buffer[1] |= match byte_count {
                0x0000000000000000..=0x007FFFFFFFFFFFFF => 0x00,
                0x0080000000000000..=0x3FFFFFFFFFFFFFFF => 0x80,
                0x4000000000000000..=0xFFFFFFFFFFFFFFFF => 0xC0,
                _ => panic!("Too high number {:x}", 0),
            };
        }

        head_size
    }

    pub fn measure(byte_count: usize) -> usize {
        Self::measure_head_size(byte_count) + byte_count
    }

    fn measure_head_size(byte_count: usize) -> usize {
        match byte_count {
            0x0000000000000000..=0x000000000000003F => 1,
            0x0000000000000040..=0x0000000000001FFF => 2,
            0x0000000000002000..=0x00000000000FFFFF => 3,
            0x0000000000100000..=0x0000000007FFFFFF => 4,
            0x0000000008000000..=0x00000003FFFFFFFF => 5,
            0x0000000400000000..=0x000001FFFFFFFFFF => 6,
            0x0000020008000000..=0x0000FFFFFFFFFFFF => 7,
            0x0001000000000000..=0x007FFFFFFFFFFFFF => 8,
            0x0080000000000000..=0x3FFFFFFFFFFFFFFF => 9,
            0x4000000000000000..=0xFFFFFFFFFFFFFFFF => 10,
            _ => panic!("Too high number {:x}", 0),
        }
    }
}

impl<'a, W: Write> Converter<'a, W> for PackedBitsCompressor<'a, W> {
    fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    fn convert(&mut self, data: &[u8]) -> std::io::Result<()> {
        let size = Self::measure(data.len());
        let mut buffer = vec![0; size];
        Self::compress(data, &mut buffer);
        self.writer.write_all(&buffer)?;
        Ok(())
    }
}

pub struct PackedBitsExtractor<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> PackedBitsExtractor<'a, W> {
    fn extract(data: &[u8], buffer: &mut [u8]) {
        let mut byte_count = 0;
        let head_size = Self::extract_head(data, &mut byte_count);
        buffer[..byte_count].copy_from_slice(&data[head_size..(head_size + byte_count)]);
    }

    fn extract_head(data: &[u8], byte_count: &mut usize) -> usize {
        let mut head_size = 0 as usize;
        for i in 0..data.len() {
            let ones = data[i].leading_ones() as usize;
            head_size += ones;

            if ones < 8 {
                break;
            }
        }

        let mut head = vec![0u8; head_size];
        head.copy_from_slice(&data[..head_size]);

        head[0] &= match head_size {
            0 => panic!("Too low size {}", head_size),
            1 => 0x3F,
            2 => 0x1F,
            3 => 0x0F,
            4 => 0x07,
            5 => 0x03,
            6 => 0x01,
            7..=10 => 0x00,
            _ => panic!("Too high size {}", head_size),
        };

        if head_size > 1 {
            head[1] &= match head_size {
                0 => panic!("Too low size {}", head_size),
                1..=7 => 0xFF,
                8 => 0x7F,
                9 => 0x3F,
                10 => 0x1F,
                _ => panic!("Too high size {}", head_size),
            };
        }

        let mut bytes = [0u8; size_of::<usize>()];
        let head = &head[(head.len().saturating_sub(size_of::<usize>()))..];
        bytes[(size_of::<usize>() - head.len())..].copy_from_slice(head);

        *byte_count = usize::from_be_bytes(bytes);

        head_size
    }

    pub fn measure(data: &[u8]) -> usize {
        let mut byte_count = 0;
        Self::extract_head(data, &mut byte_count);
        byte_count
    }
}

impl<'a, W: Write> Converter<'a, W> for PackedBitsExtractor<'a, W> {
    fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    fn convert(&mut self, data: &[u8]) -> std::io::Result<()> {
        let size = Self::measure(data);
        let mut buffer = vec![0; size];
        Self::extract(data, &mut buffer);
        self.writer.write_all(&buffer)?;
        Ok(())
    }
}

pub struct PackedBitsCompress {
    buffer: Vec<u8>,
}

impl PackedBitsCompress {
    // the maximum integer in 7 bits + 1
    const CAP_SIZE: usize = 128;

    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(Self::CAP_SIZE),
        }
    }

    pub fn push(&mut self, byte: u8, writer: &mut impl Write) -> std::io::Result<usize> {
        self.buffer.push(byte);

        if self.buffer.len() >= Self::CAP_SIZE {
            self.flush(writer)
        } else {
            Ok(0)
        }
    }

    pub fn flush(&mut self, writer: &mut impl Write) -> std::io::Result<usize> {
        let size = self.buffer.len();

        if size > 0 {
            writer.write_all(&[(size - 1) as u8])?;
            writer.write_all(&self.buffer)?;
            self.buffer.clear();

            Ok(size)
        } else {
            Ok(0)
        }
    }
}

pub struct PackedBitsExtract {
    count: usize,
}

impl PackedBitsExtract {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn is_consumed(&self) -> bool {
        return self.count <= 0;
    }

    pub fn push(&mut self, byte: u8, writer: &mut impl Write) -> std::io::Result<Option<u8>> {
        if self.is_consumed() {
            self.count = (byte as usize) + 1;
            Ok(None)
        } else {
            writer.write_all(&[byte])?;
            self.count -= 1;
            Ok(Some(byte))
        }
    }
}
