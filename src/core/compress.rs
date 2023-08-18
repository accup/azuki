use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Write},
    path::Path,
};

pub fn compress(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let mut lz77 = LZ77::new();

    while lz77.next(&mut reader, &mut writer)? {}

    Ok(())
}

/**
 * LZ77 based compression
 */
struct LZ77 {
    buffer: [u8; Self::BUFFER_SIZE],
    read_start: usize,
    compress_start: usize,
}

impl LZ77 {
    // 0                                                                     buffer size
    // v                                                                          v
    // |- work = slide + window --------------------|- extra ---------------------|
    // |- slide ----------|- window ----------------|                             |
    // |                  |. extra .....................|                         |
    //.aabaababcababbbabbbacababbabcbabcacdaecbacabcabbaccbcbaabcabcababcacabaacbb..
    //    |    |          |  |           |              |                         |
    //    |    |          |~~^=========================>|. window ................|
    //    |    |      compress index     |              |
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
    const SLIDE_SIZE: u8 = 127;
    /** the largest length of matched prefixes */
    const WINDOW_SIZE: u8 = 255;
    /** the largest working size associated with a compressing point */
    const WORK_SIZE: usize = (Self::SLIDE_SIZE as usize) + (Self::WINDOW_SIZE as usize);
    /** extra buffer size for buffering */
    const EXTRA_SIZE: usize = Self::WORK_SIZE;
    /** the buffer size */
    const BUFFER_SIZE: usize = Self::WORK_SIZE + Self::EXTRA_SIZE;

    pub fn new() -> LZ77 {
        LZ77 {
            buffer: [0u8; Self::BUFFER_SIZE],
            read_start: 0,
            compress_start: 0,
        }
    }

    pub fn next(
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
            buffer_stop - (Self::WINDOW_SIZE as usize)
        });

        for compress_index in self.compress_start..compress_stop {
            let compress_letter = buffer[compress_index];
            let slide_start = compress_index.saturating_sub(Self::SLIDE_SIZE as usize);
            let slide_stop = compress_index;
            let compare_start = compress_index;

            struct Match {
                pub left: u8,
                pub count: u8,
            }

            let mut best_match: Option<Match> = None;

            for window_start in slide_start..slide_stop {
                let index_count = (Self::WINDOW_SIZE as usize).min(buffer_stop - compress_index);

                let mut current_match = Match {
                    left: (compress_index - window_start) as u8,
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
                    writer.write_all(&[0x80u8 | left, count])?;
                } else {
                    writer.write_all(&[0u8, compress_letter])?;
                }
            } else {
                writer.write_all(&[0u8, compress_letter])?;
            }
        }

        if read_size <= 0 {
            return Ok(false);
        }

        let slide_start = compress_stop.saturating_sub(Self::SLIDE_SIZE as usize);

        buffer.copy_within(slide_start..buffer_stop, 0);

        self.read_start = buffer_stop - slide_start;
        self.compress_start = Self::SLIDE_SIZE as usize;

        Ok(true)
    }
}
