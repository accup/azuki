use std::io::Write;

use super::{
    match_layout::{Match, MatchLayout},
    packed_bits::PackedBits,
    suffix_array::U8Bucket,
    suffix_reference::SuffixReference,
};

pub struct LZ77;

#[derive(Debug)]
enum CompressMode {
    Match {
        buffer_size: usize,
        to: usize,
        match_: Match,
    },
    Pack {
        buffer_size: usize,
        to: usize,
    },
}

impl LZ77 {
    pub fn compress(data: &[u8], writer: &mut impl Write) -> std::io::Result<()> {
        let suffix = SuffixReference::from_data(data, &U8Bucket);
        let modes = {
            let mut modes = vec![];
            let mut head = 0;
            let mut index = 0;

            while index < data.len() {
                if let Some(back) = suffix.back(index) {
                    let match_ = Match {
                        left: index - back.index,
                        count: back.lcp,
                    };
                    println!("{} {}", index, back.index);
                    let m_stop = index + match_.count;

                    let m_size = MatchLayout::measure(&match_);
                    let mp_size = if head < index {
                        PackedBits::measure(&data[head..index])
                    } else {
                        0
                    };
                    let p_size = PackedBits::measure(&data[head..m_stop]);

                    if m_size + mp_size < p_size {
                        if head < index {
                            modes.push(CompressMode::Pack {
                                buffer_size: mp_size,
                                to: index,
                            });
                        }

                        modes.push(CompressMode::Match {
                            buffer_size: m_size,
                            to: m_stop,
                            match_,
                        });

                        head = m_stop;
                        index = m_stop;
                    } else {
                        index += 1;
                    }
                } else {
                    index += 1;
                }
            }

            if head < data.len() {
                let p_size = PackedBits::measure(&data[head..]);
                modes.push(CompressMode::Pack {
                    buffer_size: p_size,
                    to: data.len(),
                });
            }

            modes
        };

        let mut cursor = 0;
        for mode in modes {
            let buffer = match mode {
                CompressMode::Match {
                    buffer_size,
                    to,
                    match_,
                } => {
                    let mut buffer = vec![Default::default(); buffer_size];
                    MatchLayout::compress(&match_, &mut buffer);
                    cursor = to;
                    buffer
                }
                CompressMode::Pack { buffer_size, to } => {
                    let mut buffer = vec![Default::default(); buffer_size];
                    PackedBits::compress(&data[cursor..to], &mut buffer);
                    cursor = to;
                    buffer
                }
            };

            writer.write_all(&buffer)?;
        }

        Ok(())
    }

    pub fn extract(buffer: &[u8], writer: &mut impl Write) -> std::io::Result<()> {
        let mut memory = vec![];
        let mut head = 0;

        while head < buffer.len() {
            let buffer = &buffer[head..];

            if MatchLayout::check(&buffer) {
                let mut match_ = MatchLayout::prepare(buffer);
                let read_size = MatchLayout::extract(buffer, &mut match_);

                let cursor = memory.len();
                let back_start = cursor - match_.left;

                for i in 0..match_.count {
                    memory.push(memory[back_start + i]);
                }

                writer.write_all("M".as_bytes())?;
                writer.write_all(&memory[cursor..])?;
                head += read_size;
            } else {
                let mut data = PackedBits::prepare(buffer);
                let read_size = PackedBits::extract(buffer, &mut data);
                memory.extend_from_slice(&data);
                writer.write_all("P".as_bytes())?;
                writer.write_all(&data)?;
                head += read_size;
            }
        }

        Ok(())
    }
}
