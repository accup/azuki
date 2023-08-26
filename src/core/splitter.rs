pub type Full = Split<1>;
pub type Half = Split<2>;
pub type Quarter = Split<4>;
pub type Eighth = Split<8>;

#[derive(Clone, Debug)]
pub struct Split<const PARTITIONS: usize>;

impl<const PARTITIONS: usize> Split<PARTITIONS> {
    const BITS: u32 = u8::BITS / (PARTITIONS as u32);
    const BIT_SHIFT: u32 = Self::BITS.trailing_zeros();
    const MASK: u8 = (!(!0u32).wrapping_shl(Self::BITS)) as u8;

    const fn window(data: u8, remainder: usize) -> u8 {
        (data >> (remainder.wrapping_shl(Self::BIT_SHIFT))) & Self::MASK
    }

    fn collect(data: &[u8]) -> u8 {
        let mut byte = 0;
        for remainder in (0..data.len()).rev() {
            byte |= data[remainder] << (remainder.wrapping_shl(Self::BIT_SHIFT));
        }
        byte
    }

    pub fn unroll(data: &[u8]) -> Vec<u8> {
        (0..data.len())
            .flat_map(|at| (0..PARTITIONS).map(move |remainder| Self::window(data[at], remainder)))
            .collect()
    }

    pub fn roll(data: &[u8]) -> Vec<u8> {
        (0..data.len())
            .step_by(PARTITIONS)
            .map(|index| Self::collect(&data[index..data.len().min(index + PARTITIONS)]))
            .collect()
    }
}
