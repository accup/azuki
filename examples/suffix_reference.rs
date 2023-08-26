use std::io::stdin;

use azuki::core::{suffix_array::BucketOption, suffix_reference::SuffixReference};

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

    let suffix = SuffixReference::from_data(&chars[..], &CharBucket);

    for rank in 0..chars.len() {
        let index = suffix.index(rank);
        if let Some(back) = suffix.back(index) {
            println!("({:>8})", back.lcp);

            println!(
                "[{:>8}] {}{}",
                back.index,
                &String::from_iter(
                    chars[back.index..chars.len().min(back.index + back.lcp + 1)].into_iter()
                ),
                if back.index + back.lcp + 1 < chars.len() {
                    "..."
                } else {
                    ""
                },
            );

            println!(
                "[{:>8}] {}{}",
                index,
                &String::from_iter(chars[index..chars.len().min(index + back.lcp + 1)].into_iter()),
                if index + back.lcp + 1 < chars.len() {
                    "..."
                } else {
                    ""
                },
            );
        } else {
            println!("({:>8})", 0);

            println!("[-]");

            println!(
                "[{:>8}] {}{}",
                index,
                &String::from_iter(chars[index..chars.len().min(index + 1)].into_iter()),
                if index + 1 < chars.len() { "..." } else { "" },
            );
        }
    }
}
