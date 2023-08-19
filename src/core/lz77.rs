use std::io::{Read, Write};

use super::{
    compress::Compress,
    match_layout::{MatchLayout, MatchLayoutC3L12},
    packed_bits::PackedBitsCompress,
};

struct Match {
    pub left: usize,
    pub count: usize,
}

type LZ77MatchLayout = MatchLayoutC3L12;

/**
 * LZ77 based compression
 */
pub struct LZ77Compress {
    buffer: [u8; Self::BUFFER_SIZE],
    read_start: usize,
    compress_start: usize,
    packed_bits: PackedBitsCompress,
}

impl LZ77Compress {
    // 0                                                                     buffer size
    // v                                                                          v
    // |- work = slide + window --------------------|- extra ---------------------|
    // |- slide ----------|- window ----------------|                             |
    // |                  |. extra .....................|                         |
    //.aabaababcababbbabbbacababbabcbabcacdaecbacabcabbaccbcbaabcabcababcacabaacbb..
    //    |    |          |  |           |              |                         |
    //    |    |          |~~^=========================>|. window ................|
    //    |    |     compressing index   |              |
    //    |    |             |           |              |
    //    |    |- window ----------------|              |
    //    |~~~~^============>|           ^              |
    //    |  window start    |      window stop         |
    //    |                  |                          |
    //    |. slide ..........|. window .................|
    //    ^                  ^
    // slide start       slide stop
    //                       ^
    //                  compare start

    /** the largest offset of matched prefixes */
    const SLIDE_SIZE: usize = LZ77MatchLayout::MAX_LEFT;
    /** the distance between slide stop and compressing index */
    const SLIDE_OFFSET: usize = 0;
    /** the largest length of matched prefixes */
    const WINDOW_SIZE: usize = LZ77MatchLayout::MAX_COUNT;
    /** the largest working size associated with a compressing index */
    const WORK_SIZE: usize = Self::SLIDE_SIZE + Self::SLIDE_OFFSET + Self::WINDOW_SIZE;
    /** extra buffer size for buffering */
    const EXTRA_SIZE: usize = Self::WORK_SIZE;
    /** the buffer size */
    const BUFFER_SIZE: usize = Self::WORK_SIZE + Self::EXTRA_SIZE;

    pub fn new() -> LZ77Compress {
        LZ77Compress {
            buffer: [0u8; Self::BUFFER_SIZE],
            read_start: 0,
            compress_start: 0,
            packed_bits: PackedBitsCompress::new(),
        }
    }
}

impl Compress for LZ77Compress {
    fn compress_next(
        &mut self,
        reader: &mut impl Read,
        writer: &mut impl Write,
    ) -> std::io::Result<bool> {
        let read_size = reader.read(&mut self.buffer[self.read_start..])?;
        let buffer_stop = self.read_start + read_size;
        let buffer = &mut self.buffer[..buffer_stop];

        let compress_stop = self.compress_start.max(if read_size <= 0 {
            // last compression
            buffer_stop
        } else {
            // first and intermediate compression
            buffer_stop - (Self::WINDOW_SIZE)
        });

        let mut compress_index = self.compress_start;

        while compress_index < compress_stop {
            let compress_letter = buffer[compress_index];
            let slide_stop = compress_index.saturating_sub(Self::SLIDE_OFFSET);
            let slide_start = slide_stop.saturating_sub(Self::SLIDE_SIZE);
            let compare_start = compress_index;

            let mut best_match: Option<Match> = None;

            for window_start in slide_start..slide_stop {
                let index_count = (Self::WINDOW_SIZE).min(buffer_stop - compress_index);

                let mut current_match = Match {
                    left: slide_stop - window_start,
                    count: 0,
                };

                for index in 0..index_count {
                    let compare_letter = buffer[compare_start + index];
                    let window_letter = buffer[window_start + index];

                    if compare_letter != window_letter {
                        break;
                    }

                    current_match.count += 1;
                }

                if let Some(Match {
                    count: best_count, ..
                }) = &best_match
                {
                    if current_match.count > *best_count {
                        best_match = Some(current_match);
                    }
                } else if current_match.count > 0 {
                    best_match = Some(current_match);
                }
            }

            if let Some(Match { left, count }) = best_match {
                if count >= 3 {
                    self.packed_bits.flush(writer)?;
                    LZ77MatchLayout::write(left, count, writer)?;
                    compress_index += count;
                } else {
                    self.packed_bits.push(compress_letter, writer)?;
                    compress_index += 1;
                }
            } else {
                self.packed_bits.push(compress_letter, writer)?;
                compress_index += 1;
            }
        }

        if read_size <= 0 {
            self.packed_bits.flush(writer)?;

            return Ok(false);
        }

        let slide_start = compress_index.saturating_sub(Self::SLIDE_SIZE);

        buffer.copy_within(slide_start..buffer_stop, 0);

        self.read_start = buffer_stop - slide_start;
        self.compress_start = Self::SLIDE_SIZE;

        Ok(true)
    }
}
