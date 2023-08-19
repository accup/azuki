use std::io::Write;

pub trait MatchLayout {
    const MAX_LEFT: usize;
    const MAX_COUNT: usize;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<()>;
}

pub struct MatchLayoutC2L13;

impl MatchLayout for MatchLayoutC2L13 {
    const MAX_LEFT: usize = 8192;
    const MAX_COUNT: usize = 4;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<()> {
        // 1CCLLLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = (left - 1).to_le_bytes();
        let le_count = (count - 1).to_le_bytes();
        writer.write_all(&[0x80u8 | (le_count[0] << 5) | le_left[1], le_left[0]])?;

        Ok(())
    }
}

pub struct MatchLayoutC3L12;

impl MatchLayout for MatchLayoutC3L12 {
    const MAX_LEFT: usize = 4096;
    const MAX_COUNT: usize = 8;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<()> {
        // 1CCCLLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = (left - 1).to_le_bytes();
        let le_count = (count - 1).to_le_bytes();
        writer.write_all(&[0x80u8 | (le_count[0] << 4) | le_left[1], le_left[0]])?;

        Ok(())
    }
}

pub struct MatchLayoutC4L11;

impl MatchLayout for MatchLayoutC4L11 {
    const MAX_LEFT: usize = 2048;
    const MAX_COUNT: usize = 16;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<()> {
        // 1CCCCLLL LLLLLLLL (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = (left - 1).to_le_bytes();
        let le_count = (count - 1).to_le_bytes();
        writer.write_all(&[0x80u8 | (le_count[0] << 3) | le_left[1], le_left[0]])?;

        Ok(())
    }
}

pub struct MatchLayoutL7C8;

impl MatchLayout for MatchLayoutL7C8 {
    const MAX_LEFT: usize = 128;
    const MAX_COUNT: usize = 256;

    fn write(left: usize, count: usize, writer: &mut impl Write) -> std::io::Result<()> {
        // 1LLLLLLL CCCCCCCC (2 bytes)
        // 1: matched flag
        // L: left
        // C: count
        let le_left = (left - 1).to_le_bytes();
        let le_count = (count - 1).to_le_bytes();
        writer.write_all(&[0x80u8 | le_count[0], le_left[0]])?;

        Ok(())
    }
}
