use std::io::stdin;

use azuki::core::suffix_array::{BucketOption, SuffixArrayBucket, SuffixType};

struct CharBucket;

impl BucketOption<char> for CharBucket {
    fn size(&self) -> usize {
        return char::MAX as usize;
    }

    fn bucket_index(&self, value: &char) -> usize {
        return (*value) as usize;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim_end();

    let chars = input.chars().collect::<Vec<_>>();
    let mut types = vec![SuffixType::L; chars.len()];
    for index in (1..chars.len()).rev() {
        types[index - 1] = if chars[index - 1] == chars[index] {
            types[index]
        } else if chars[index - 1] < chars[index] {
            SuffixType::S
        } else {
            SuffixType::L
        };
    }

    let mut bucket = SuffixArrayBucket::new(&chars, &types, &CharBucket);
    for (index, char) in chars.iter().enumerate() {
        bucket.push(index, char, SuffixType::L);
    }

    for bin in 0..bucket.num_bins() {
        if bucket.len_l_bin(bin) + bucket.len_s_bin(bin) <= 0 {
            continue;
        }

        println!("[{}]", bin);
        println!(
            "    ({}) {:?}",
            bucket.len_l_bin(bin),
            (0..bucket.len_l_bin(bin))
                .map(|rank| bucket.l_index_by_rank(bin, rank))
                .collect::<Vec<_>>()
        );
        println!(
            "    ({}) {:?}",
            bucket.len_s_bin(bin),
            (0..bucket.len_s_bin(bin))
                .map(|rank| bucket.s_index_by_rank(bin, rank))
                .collect::<Vec<_>>()
        );
        println!(
            "    ({}) {:?}",
            bucket.len_l_bin(bin),
            (0..bucket.len_l_bin(bin))
                .map(|rank| bucket.l_index_by_rev_rank(bin, rank))
                .collect::<Vec<_>>()
        );
        println!(
            "    ({}) {:?}",
            bucket.len_s_bin(bin),
            (0..bucket.len_s_bin(bin))
                .map(|rank| bucket.s_index_by_rev_rank(bin, rank))
                .collect::<Vec<_>>()
        );
    }
}
