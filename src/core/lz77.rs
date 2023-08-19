use std::io::{Read, Write};

use super::{
    compress::Compress,
    extract::Extract,
    match_layout::{Match, MatchLayout, MatchLayoutC3L12},
    packed_bits::{PackedBitsCompress, PackedBitsExtract},
};

type LZ77MatchLayout = MatchLayoutC3L12;

/**
 * LZ77 based compression
 */
pub struct LZ77Compress {
    buffer: [u8; Self::BUFFER_SIZE],
    read_start: usize,
    compress_start: usize,
    packed_bits: PackedBitsCompress,
    bytes_read: usize,
    bytes_written: usize,
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
    const WINDOW_SIZE: usize = LZ77MatchLayout::MAX_COUNT;
    /** the largest working size associated with a compressing index */
    const WORK_SIZE: usize = Self::SLIDE_SIZE + Self::WINDOW_SIZE;
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
            bytes_read: 0,
            bytes_written: 0,
        }
    }

    pub fn bytes_read(&self) -> usize {
        self.bytes_read
    }

    pub fn bytes_written(&self) -> usize {
        self.bytes_written
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

        self.bytes_read = self.bytes_read.saturating_add(read_size);

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
            let slide_stop = compress_index;
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
                    self.bytes_written += self.packed_bits.flush(writer)?;
                    self.bytes_written += LZ77MatchLayout::write(left, count, writer)?;
                    compress_index += count;
                } else {
                    self.bytes_written += self.packed_bits.push(compress_letter, writer)?;
                    compress_index += 1;
                }
            } else {
                self.bytes_written += self.packed_bits.push(compress_letter, writer)?;
                compress_index += 1;
            }
        }

        if read_size <= 0 {
            self.bytes_written += self.packed_bits.flush(writer)?;

            return Ok(false);
        }

        let slide_start = compress_index.saturating_sub(Self::SLIDE_SIZE);

        buffer.copy_within(slide_start..buffer_stop, 0);

        self.read_start = buffer_stop - slide_start;
        self.compress_start = Self::SLIDE_SIZE;

        Ok(true)
    }
}

/**
 * LZ77 based extraction
 */
pub struct LZ77Extract {
    history: [u8; Self::HISTORY_SIZE],
    write_start: usize,
    packed_bits: PackedBitsExtract,
    bytes_read: usize,
    bytes_written: usize,
}

impl LZ77Extract {
    // 0                                                                    history size
    // v                                                                          v
    // |- work = slide + window --------------------|- extra ---------------------|
    // |- slide ----------|- window ----------------|                             |
    // |                  |. extra .....................|                         |
    //.aabaababcababbbabbbacababbabcbabcacdaecbacabcabbaccbcbaabcabcababcacabaacbb..

    /** the largest offset of matched prefixes */
    const SLIDE_SIZE: usize = LZ77MatchLayout::MAX_LEFT;
    /** the largest length of matched prefixes */
    const WINDOW_SIZE: usize = LZ77MatchLayout::MAX_COUNT;
    /** the largest working size associated with a compressing index */
    const WORK_SIZE: usize = Self::SLIDE_SIZE + Self::WINDOW_SIZE;
    /** extra buffer size for buffering */
    const EXTRA_SIZE: usize = Self::WORK_SIZE;
    /** the buffer size */
    const HISTORY_SIZE: usize = Self::WORK_SIZE + Self::EXTRA_SIZE;

    pub fn new() -> LZ77Extract {
        LZ77Extract {
            history: [0u8; Self::HISTORY_SIZE],
            write_start: 0,
            packed_bits: PackedBitsExtract::new(),
            bytes_read: 0,
            bytes_written: 0,
        }
    }

    pub fn bytes_read(&self) -> usize {
        self.bytes_read
    }

    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

impl Extract for LZ77Extract {
    fn extract_next(
        &mut self,
        reader: &mut impl Read,
        writer: &mut impl Write,
    ) -> std::io::Result<bool> {
        let mut buffer = [0u8; 1];
        let read_size = reader.read(&mut buffer)?;
        self.bytes_read += read_size;

        if read_size <= 0 {
            return Ok(false);
        }

        let [byte0] = buffer;

        if self.packed_bits.is_consumed() {
            let read_size = reader.read(&mut buffer)?;
            self.bytes_read += read_size;

            if read_size > 0 {
                let [byte1] = buffer;

                if LZ77MatchLayout::check(&[byte0, byte1]) {
                    let Match { left, count } = LZ77MatchLayout::read(&[byte0, byte1]);
                    let refer_start = self.write_start.saturating_sub(left);
                    let write_stop = self.write_start + count;

                    for index in 0..count {
                        self.history[self.write_start + index] = self.history[refer_start + index];
                    }

                    writer.write_all(&self.history[self.write_start..write_stop])?;
                    self.write_start += count;
                    self.bytes_written += count;
                } else {
                    if let Some(byte) = self.packed_bits.push(byte0, writer)? {
                        self.history[self.write_start] = byte;
                        self.write_start += 1;
                        self.bytes_written += 1;
                    }

                    if let Some(byte) = self.packed_bits.push(byte1, writer)? {
                        self.history[self.write_start] = byte;
                        self.write_start += 1;
                        self.bytes_written += 1;
                    }
                }
            } else {
                if let Some(byte) = self.packed_bits.push(byte0, writer)? {
                    self.history[self.write_start] = byte;
                    self.write_start += 1;
                    self.bytes_written += 1;
                }
            }
        } else {
            if let Some(byte) = self.packed_bits.push(byte0, writer)? {
                self.history[self.write_start] = byte;
                self.write_start += 1;
                self.bytes_written += 1;
            }
        };

        let slide_start = self.write_start.saturating_sub(Self::SLIDE_SIZE);

        self.history.copy_within(slide_start..self.write_start, 0);

        self.write_start -= slide_start;

        Ok(true)
    }
}
