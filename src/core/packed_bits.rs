use std::io::Write;

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
