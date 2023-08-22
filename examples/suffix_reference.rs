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

    let suffix_ref = SuffixReference::from_data(&chars[..], &CharBucket);

    loop {
        let mut index_input = String::new();
        stdin().read_line(&mut index_input).unwrap();

        let Ok(index) = index_input.trim().parse() else { break };
        let Some(back) = suffix_ref.back(index) else { continue };
        let Some(lcp) = suffix_ref.back_lcp(index) else { continue };

        println!("({:>8})", lcp);

        println!(
            "[{:>8}] {}{}",
            back,
            &String::from_iter(chars[back..chars.len().min(back + 8)].into_iter()),
            if index + 9 < chars.len() { "..." } else { "" },
        );

        println!(
            "[{:>8}] {}{}",
            index,
            &String::from_iter(chars[index..chars.len().min(index + 8)].into_iter()),
            if index + 9 < chars.len() { "..." } else { "" },
        );
    }
}
