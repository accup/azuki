use std::io::Write;

use super::{
    converter::Converter,
    head::{Head, LeadingOne},
};

pub struct PackedBitsCompressor<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write + 'a> PackedBitsCompressor<'a, W> {
    fn compress(data: &[u8], buffer: &mut [u8]) {
        let head_size = Head::<LeadingOne>::compress(data.len(), buffer);
        buffer[head_size..(head_size + data.len())].copy_from_slice(data);
    }

    pub fn measure(byte_count: usize) -> usize {
        Head::<LeadingOne>::measure(byte_count) + byte_count
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
        let head_size = Head::<LeadingOne>::extract(data, &mut byte_count);
        buffer[..byte_count].copy_from_slice(&data[head_size..(head_size + byte_count)]);
    }

    pub fn measure(data: &[u8]) -> usize {
        let mut byte_count = 0;
        Head::<LeadingOne>::extract(data, &mut byte_count);
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
