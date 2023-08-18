use std::io::Write;

pub struct PackedBitsCompress {
    buffer: Vec<u8>,
}

impl PackedBitsCompress {
    // the maximum integer in 7 bits
    const CAP_SIZE: usize = 127;

    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(Self::CAP_SIZE),
        }
    }

    pub fn push(&mut self, byte: u8, writer: &mut impl Write) -> std::io::Result<()> {
        self.buffer.push(byte);

        if self.buffer.len() >= Self::CAP_SIZE {
            self.flush(writer)?;
        }

        Ok(())
    }

    pub fn flush(&mut self, writer: &mut impl Write) -> std::io::Result<()> {
        if self.buffer.len() > 0 {
            writer.write_all(&[self.buffer.len() as u8])?;
            writer.write_all(&self.buffer)?;
            self.buffer.clear();
        }

        Ok(())
    }
}
