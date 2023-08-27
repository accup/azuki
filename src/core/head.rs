use std::{marker::PhantomData, mem::size_of};

pub trait HeadType {
    fn mark(byte_count: usize, buffer: &mut [u8]);

    fn mask(head_size: usize, buffer: &mut [u8]);

    fn size(byte_count: usize) -> usize;

    fn count(buffer: &[u8]) -> usize;
}

fn leading_mask(head_size: usize, buffer: &mut [u8]) {
    buffer[0] &= match head_size {
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
        buffer[1] &= match head_size {
            0 => panic!("Too low size {}", head_size),
            1..=7 => 0xFF,
            8 => 0x7F,
            9 => 0x3F,
            10 => 0x1F,
            _ => panic!("Too high size {}", head_size),
        };
    }
}

fn leading_size(byte_count: usize) -> usize {
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

pub struct LeadingZero;

impl HeadType for LeadingZero {
    fn mark(byte_count: usize, buffer: &mut [u8]) {
        buffer[0] |= match byte_count {
            0x0000000000000000..=0x000000000000003F => 0x40,
            0x0000000000000040..=0x0000000000001FFF => 0x20,
            0x0000000000002000..=0x00000000000FFFFF => 0x10,
            0x0000000000100000..=0x0000000007FFFFFF => 0x08,
            0x0000000008000000..=0x00000003FFFFFFFF => 0x04,
            0x0000000400000000..=0x000001FFFFFFFFFF => 0x02,
            0x0000020008000000..=0x0000FFFFFFFFFFFF => 0x01,
            0x0001000000000000..=0xFFFFFFFFFFFFFFFF => 0x00,
            _ => panic!("Too high number {:x}", 0),
        };

        if buffer.len() > 1 {
            buffer[1] |= match byte_count {
                0x0000000000000000..=0x0000FFFFFFFFFFFF => 0x00,
                0x0001000000000000..=0x007FFFFFFFFFFFFF => 0x80,
                0x0080000000000000..=0x3FFFFFFFFFFFFFFF => 0x40,
                0x4000000000000000..=0xFFFFFFFFFFFFFFFF => 0x20,
                _ => panic!("Too high number {:x}", 0),
            };
        }
    }

    fn mask(head_size: usize, buffer: &mut [u8]) {
        leading_mask(head_size, buffer);
    }

    fn size(byte_count: usize) -> usize {
        leading_size(byte_count)
    }

    fn count(buffer: &[u8]) -> usize {
        let mut count: usize = 0;

        for i in 0..buffer.len() {
            let zeros = buffer[i].leading_zeros() as usize;
            count += zeros;

            if zeros < 8 {
                break;
            }
        }

        count
    }
}

pub struct LeadingOne;

impl HeadType for LeadingOne {
    fn mark(byte_count: usize, buffer: &mut [u8]) {
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

        if buffer.len() > 1 {
            buffer[1] |= match byte_count {
                0x0000000000000000..=0x0000FFFFFFFFFFFF => 0x00,
                0x0080000000000000..=0x3FFFFFFFFFFFFFFF => 0x80,
                0x4000000000000000..=0xFFFFFFFFFFFFFFFF => 0xC0,
                _ => panic!("Too high number {:x}", 0),
            };
        }
    }

    fn mask(head_size: usize, buffer: &mut [u8]) {
        leading_mask(head_size, buffer);
    }

    fn size(byte_count: usize) -> usize {
        leading_size(byte_count)
    }

    fn count(buffer: &[u8]) -> usize {
        let mut count: usize = 0;

        for i in 0..buffer.len() {
            let ones = buffer[i].leading_ones() as usize;
            count += ones;

            if ones < 8 {
                break;
            }
        }

        count
    }
}

pub struct Common;

impl HeadType for Common {
    fn mark(byte_count: usize, buffer: &mut [u8]) {
        buffer[0] |= match byte_count {
            0x0000000000000000..=0x000000000000007F => 0x00,
            0x0000000000000080..=0x0000000000003FFF => 0x80,
            0x0000000000004000..=0x00000000001FFFFF => 0xC0,
            0x0000000000200000..=0x000000000FFFFFFF => 0xE0,
            0x0000000010000000..=0x00000007FFFFFFFF => 0xF0,
            0x0000000800000000..=0x000003FFFFFFFFFF => 0xF8,
            0x0000040008000000..=0x0001FFFFFFFFFFFF => 0xFC,
            0x0002000008000000..=0x00FFFFFFFFFFFFFF => 0xFE,
            0x0100000000000000..=0xFFFFFFFFFFFFFFFF => 0xFF,
            _ => panic!("Too high number {:x}", 0),
        };

        if buffer.len() > 1 {
            buffer[1] |= match byte_count {
                0x0000000000000000..=0x00FFFFFFFFFFFFFF => 0x00,
                0x8000000000000000..=0xFFFFFFFFFFFFFFFF => 0x80,
                _ => panic!("Too high number {:x}", 0),
            };
        }
    }

    fn mask(head_size: usize, buffer: &mut [u8]) {
        buffer[0] &= match head_size {
            0 => panic!("Too low size {}", head_size),
            1 => 0x7F,
            2 => 0x3F,
            3 => 0x1F,
            4 => 0x0F,
            5 => 0x07,
            6 => 0x03,
            7 => 0x01,
            8..=9 => 0x00,
            _ => panic!("Too high size {}", head_size),
        };

        if head_size > 1 {
            buffer[1] &= match head_size {
                0 => panic!("Too low size {}", head_size),
                1..=8 => 0xFF,
                9 => 0x7F,
                _ => panic!("Too high size {}", head_size),
            };
        }
    }

    fn size(byte_count: usize) -> usize {
        match byte_count {
            0x0000000000000000..=0x000000000000007F => 1,
            0x0000000000000080..=0x0000000000003FFF => 2,
            0x0000000000004000..=0x00000000001FFFFF => 3,
            0x0000000000200000..=0x000000000FFFFFFF => 4,
            0x0000000010000000..=0x00000007FFFFFFFF => 5,
            0x0000000800000000..=0x000003FFFFFFFFFF => 6,
            0x0000040008000000..=0x0001FFFFFFFFFFFF => 7,
            0x0002000008000000..=0x00FFFFFFFFFFFFFF => 8,
            0x0100000000000000..=0xFFFFFFFFFFFFFFFF => 9,
            _ => panic!("Too high number {:x}", 0),
        }
    }

    fn count(buffer: &[u8]) -> usize {
        let mut count: usize = 1;

        for i in 0..buffer.len() {
            let ones = buffer[i].leading_ones() as usize;
            count += ones;

            if ones < 8 {
                break;
            }
        }

        count
    }
}

pub struct Head<H: HeadType> {
    phantom: PhantomData<H>,
}

impl<H: HeadType> Head<H> {
    pub fn measure(data: &usize) -> usize {
        H::size(*data)
    }

    pub fn compress(data: &usize, buffer: &mut [u8]) -> usize {
        let head_size = H::size(*data);

        let bytes = data.to_be_bytes();
        let bytes = &bytes[(size_of::<usize>().saturating_sub(head_size))..];

        buffer[(head_size - bytes.len())..head_size].copy_from_slice(bytes);

        H::mark(*data, buffer);

        head_size
    }

    pub fn prepare(_: &[u8]) -> usize {
        Default::default()
    }

    pub fn extract(buffer: &[u8], data: &mut usize) -> usize {
        let head_size = H::count(buffer);

        let mut head = vec![0u8; head_size];
        head.copy_from_slice(&buffer[..head_size]);

        H::mask(head_size, &mut head);

        let mut bytes = [0u8; size_of::<usize>()];
        let head = &head[(head.len().saturating_sub(size_of::<usize>()))..];
        bytes[(size_of::<usize>() - head.len())..].copy_from_slice(head);

        *data = usize::from_be_bytes(bytes);

        head_size
    }
}
