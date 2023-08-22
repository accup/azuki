use std::io::stdin;

use azuki::core::suffix_array::{lcp_array, rank_array, suffix_array, BucketOption};

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
    let sa = suffix_array(&chars, &CharBucket);
    let rank = rank_array(&sa);
    let lcp = lcp_array(&chars, &sa, &rank);

    for (&index, &lcp) in sa.iter().zip(lcp.iter()) {
        println!(
            "{:>8} ({:>8}): {}{}",
            index,
            lcp,
            &String::from_iter(chars[index..chars.len().min(index + 8)].into_iter()),
            if index + 9 < chars.len() { "..." } else { "" },
        );
    }
}
