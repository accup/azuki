use std::io::{stdin, Read};

use azuki::core::{
    bwt::bwt,
    suffix_array::{suffix_array, BucketOption},
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
    stdin().read_to_string(&mut input).unwrap();

    let chars = input.chars().collect::<Vec<_>>();
    let sa = suffix_array(&chars, &CharBucket);
    let bwt = bwt(&chars, &sa);

    println!("{}", String::from_iter(bwt));
}
