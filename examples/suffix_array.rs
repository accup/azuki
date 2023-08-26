use std::io::stdin;

use azuki::core::{
    suffix_array::{lcp_array, rank_array, suffix_array, BucketOption, SuffixType},
    suffix_reference::back_array,
};

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
    let back = back_array(&sa, &lcp);

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

    for (rank, &index) in sa.iter().enumerate() {
        let stop = index + 8;
        let stop = chars.len().min(stop);
        println!(
            "{:>8} ({:>8}) [{}{:>7} ({}{:>7})]: {}{}",
            index,
            lcp[rank],
            back[index].map_or(Default::default(), |b| format!(
                "{}",
                if index <= b.index { "!" } else { " " }
            )),
            back[index].map_or(Default::default(), |b| format!("{}", b.index)),
            back[index].map_or(Default::default(), |b| format!(
                "{}",
                if lcp[rank] < b.lcp && lcp.get(rank + 1).map_or(false, |l| *l < b.lcp) {
                    "!"
                } else {
                    " "
                }
            )),
            back[index].map_or(Default::default(), |b| format!("{}", b.lcp)),
            String::from_iter(chars[index..stop].into_iter()),
            if stop < chars.len() { "..." } else { "" },
        );
    }
}
