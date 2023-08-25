use super::head::{Head, LeadingOne};

pub struct PackedBits;

impl PackedBits {
    pub fn measure(data: &[u8]) -> usize {
        let byte_count = data.len();
        Head::<LeadingOne>::measure(&byte_count) + byte_count
    }

    pub fn compress(data: &[u8], buffer: &mut [u8]) -> usize {
        let byte_count = data.len();
        let cursor = Head::<LeadingOne>::compress(&byte_count, buffer);
        buffer[cursor..(cursor + byte_count)].copy_from_slice(data);
        cursor + byte_count
    }

    pub fn prepare(buffer: &[u8]) -> Vec<u8> {
        let mut byte_count = 0;
        Head::<LeadingOne>::extract(buffer, &mut byte_count);
        vec![Default::default(); byte_count]
    }

    pub fn extract(buffer: &[u8], data: &mut [u8]) -> usize {
        let mut byte_count = 0;
        let cursor = Head::<LeadingOne>::extract(buffer, &mut byte_count);
        data[..byte_count].copy_from_slice(&buffer[cursor..(cursor + byte_count)]);
        cursor + byte_count
    }
}
