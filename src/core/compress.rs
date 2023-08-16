use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter, Write},
    path::Path,
};

pub fn compress(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    // LZ77 based compression

    // |- work = slide + window - 1 ----------------|- extra -----------------|
    // |- slide ----------|- tail ------------------|                         |
    //.aabaababcababbbabbbacababbabcbabcacdaecbacabcabbaccbcbaabcabcabcabaacbb..
    // |    |             |           |             |                         |
    // |    |             ^           |             |                         |
    // |    |     compressing point   |             |                         |
    // |    |             |           |             |                         |
    // |    |- window ----------------|             |                         |
    // |____^============>|. window ................|                         |
    // |   window start   ^           ^                                       |
    // |              slide stop   window stop                                |
    // |. extra .................|. work .....................................|
    // ^========================>|
    // slide start               ^
    //                       work stop

    /** the largest offset of matched prefixes */
    const SLIDE_SIZE: u8 = 127;
    /** the largest length of matched prefixes */
    const WINDOW_SIZE: u8 = 255;
    /** the largest working size associated with a compressing point */
    const WORK_SIZE: usize = (SLIDE_SIZE as usize) + (WINDOW_SIZE as usize) - 1;
    /** extra buffer size for buffering */
    const EXTRA_SIZE: usize = WORK_SIZE;
    /** the buffer size */
    const BUFFER_SIZE: usize = WORK_SIZE + EXTRA_SIZE;

    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut read_index: usize = 0;

    loop {
        let read_size = reader.read(&mut buffer[read_index..])?;
        let buffer_stop = read_index + read_size;
        read_index = buffer_stop;

        if read_size <= 0 {
            // last compression

            let work_stop = buffer_stop;

            for slide_start in 0..work_stop {
                let slide_stop = slide_start + (SLIDE_SIZE as usize);
                let slide_stop = slide_stop.min(buffer_stop);
                let compress_start = slide_stop;
                let compress_letter = buffer[compress_start];

                struct Match {
                    pub left: u8,
                    pub count: u8,
                }

                let mut best_match: Option<Match> = None;

                for window_start in slide_start..slide_stop {
                    let index_stop = (WINDOW_SIZE as usize).min(buffer_stop - window_start);
                    let mut current_match = Match {
                        left: (compress_start - window_start) as u8,
                        count: 0,
                    };

                    for index in 0..index_stop {
                        let compare_letter = buffer[compress_start + index];
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
                    } else {
                        best_match = Some(current_match);
                    }
                }

                if let Some(Match { left, count }) = best_match {
                    writer.write(&[0x80u8 | left, count])?;
                } else {
                    writer.write(&[0u8, compress_letter])?;
                }
            }

            break;
        } else if buffer_stop > (SLIDE_SIZE as usize) {
            // intermediate compression

            let work_stop = (EXTRA_SIZE as usize).min(buffer_stop - WORK_SIZE);

            for slide_start in 0..work_stop {
                let slide_stop = slide_start + (SLIDE_SIZE as usize);
                let compress_start = slide_stop;
                let compress_letter = buffer[compress_start];

                struct Match {
                    pub left: u8,
                    pub count: u8,
                }

                let mut best_match: Option<Match> = None;

                for window_start in slide_start..slide_stop {
                    let mut current_match = Match {
                        left: (compress_start - window_start) as u8,
                        count: 0,
                    };

                    for index in 0..(WINDOW_SIZE as usize) {
                        let compare_letter = buffer[compress_start + index];
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
                    } else {
                        best_match = Some(current_match);
                    }
                }

                if let Some(Match { left, count }) = best_match {
                    writer.write(&[0x80u8 | left, count])?;
                } else {
                    writer.write(&[0u8, compress_letter])?;
                }
            }

            buffer.copy_within(work_stop..buffer_stop, 0);
            read_index = buffer_stop - work_stop;
        }
    }

    Ok(())
}
