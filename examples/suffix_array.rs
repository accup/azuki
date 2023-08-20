use std::io::stdin;

use azuki::core::suffix_array::{suffix_array, BucketOption};

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

    let sa = suffix_array(&input.chars().collect::<Vec<_>>(), &CharBucket);
    println!("{:?}", sa);
}
