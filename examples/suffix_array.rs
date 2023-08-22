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

#[derive(Clone, Copy)]
enum Type {
    L,
    S,
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim_end();

    let chars = input.chars().collect::<Vec<_>>();
    let sa = suffix_array(&chars, &CharBucket);
    let rank = rank_array(&sa);
    let lcp = lcp_array(&chars, &sa, &rank);

    let mut types = vec![Type::L; chars.len()];
    for index in (1..chars.len()).rev() {
        types[index - 1] = if chars[index - 1] == chars[index] {
            types[index]
        } else if chars[index - 1] < chars[index] {
            Type::S
        } else {
            Type::L
        };
    }

    for (&index, &lcp) in sa.iter().zip(lcp.iter()) {
        println!(
            "{:>8} ({:>8}): {:<13} {}",
            index,
            lcp,
            format!(
                "{}{}",
                String::from_iter(chars[index..chars.len().min(index + 8)].into_iter()),
                if index + 8 < chars.len() { "..." } else { "" }
            ),
            match types[index] {
                Type::L => "L",
                Type::S => "S",
            }
        );
    }
}
