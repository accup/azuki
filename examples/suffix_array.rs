use std::{fmt::Debug, io::stdin};

struct CharBucket;

impl BucketOption<char> for CharBucket {
    fn size(&self) -> usize {
        return char::MAX as usize;
    }

    fn bucket_index(&self, value: &char) -> usize {
        return (*value) as usize;
    }
}
use std::collections::BTreeMap;

pub trait BucketOption<T> {
    fn size(&self) -> usize;
    fn bucket_index(&self, value: &T) -> usize;
}

struct U8Bucket;

impl BucketOption<u8> for U8Bucket {
    fn size(&self) -> usize {
        return 256;
    }

    fn bucket_index(&self, value: &u8) -> usize {
        return (*value) as usize;
    }
}

struct IndexBucket {
    size: usize,
}

impl BucketOption<usize> for IndexBucket {
    fn size(&self) -> usize {
        return self.size;
    }

    fn bucket_index(&self, value: &usize) -> usize {
        return *value;
    }
}

#[derive(Clone)]
struct SuffixArrayBucket {
    /** Forward order */
    pub l_typed: Vec<usize>,
    /** Backward order */
    pub s_typed: Vec<usize>,
}

impl SuffixArrayBucket {
    pub fn new() -> Self {
        Self {
            l_typed: Vec::new(),
            s_typed: Vec::new(),
        }
    }
}

#[derive(Copy, Clone)]
enum Type {
    S,
    L,
}

fn induced_sort<T: PartialEq + PartialOrd, B: BucketOption<T>>(
    data: &[T],
    types: &[Type],
    buckets: &mut Vec<SuffixArrayBucket>,
    bucket_option: &B,
) {
    // insert the last L-typed item
    if data.len() > 0 {
        let index = data.len() - 1;

        match types[index] {
            Type::L => {
                let bucket_index = bucket_option.bucket_index(&data[index]);
                buckets[bucket_index].l_typed.push(index);
            }
            _ => {}
        }
    }

    // insert all other L-typed items
    for bucket_index in 0..buckets.len() {
        // L in forward-forward order
        {
            let mut l_index = 0usize;

            while l_index < buckets[bucket_index].l_typed.len() {
                let index = buckets[bucket_index].l_typed[l_index];

                if index > 0 {
                    match types[index - 1] {
                        Type::L => {
                            let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                            buckets[bucket_index].l_typed.push(index - 1);
                        }
                        _ => {}
                    }
                }

                l_index += 1;
            }
        }

        // S in backward-backward order
        {
            let mut rev_s_index = 0usize;

            while rev_s_index < buckets[bucket_index].s_typed.len() {
                let index = {
                    let s_typed = &buckets[bucket_index].s_typed;
                    s_typed[s_typed.len() - rev_s_index - 1]
                };

                if index > 0 {
                    match types[index - 1] {
                        Type::L => {
                            let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                            buckets[bucket_index].l_typed.push(index - 1);
                        }
                        _ => {}
                    }
                }

                rev_s_index += 1;
            }
        }
    }

    // Clear S-typed items from buckets
    for bucket_index in 0..buckets.len() {
        buckets[bucket_index].s_typed.clear();
    }

    // insert all S-typed items
    for bucket_index in (0..buckets.len()).rev() {
        // S in backward-forward order
        {
            let mut s_index = 0usize;

            while s_index < buckets[bucket_index].s_typed.len() {
                let index = buckets[bucket_index].s_typed[s_index];

                if index > 0 {
                    match types[index - 1] {
                        Type::S => {
                            let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                            buckets[bucket_index].s_typed.push(index - 1);
                        }
                        _ => {}
                    }
                }

                s_index += 1;
            }
        }

        // L in forward-backward order
        {
            let mut rev_l_index = 0usize;

            while rev_l_index < buckets[bucket_index].l_typed.len() {
                let index = {
                    let l_typed = &buckets[bucket_index].l_typed;
                    l_typed[l_typed.len() - rev_l_index - 1]
                };

                if index > 0 {
                    match types[index - 1] {
                        Type::S => {
                            let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                            buckets[bucket_index].s_typed.push(index - 1);
                        }
                        _ => {}
                    }
                }

                rev_l_index += 1;
            }
        }
    }
}

pub fn suffix_array<T: PartialEq + PartialOrd, B: BucketOption<T>>(
    data: &[T],
    bucket_option: &B,
) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let mut types = vec![Type::L; data.len()];

    for index in (1..data.len()).rev() {
        types[index - 1] = if data[index - 1] == data[index] {
            types[index]
        } else if data[index - 1] < data[index] {
            Type::S
        } else {
            Type::L
        };
    }

    let mut lms_orders: Vec<usize> = vec![0; data.len()];
    let mut lms_indices: Vec<usize> = Vec::new();
    let mut buckets: Vec<SuffixArrayBucket> = vec![SuffixArrayBucket::new(); bucket_option.size()];

    // collect left-most S-typed indices
    for index in 1..data.len() {
        match types[index - 1] {
            Type::L => match types[index] {
                Type::S => {
                    lms_orders[index] = lms_indices.len();
                    lms_indices.push(index);
                }
                _ => {}
            },
            _ => {}
        }
    }

    // insert left-most S-typed indices into S-typed buckets
    for &index in lms_indices.iter() {
        let bucket_index = bucket_option.bucket_index(&data[index]);
        buckets[bucket_index].s_typed.push(index);
    }

    // Induced sort
    induced_sort(data, &types, &mut buckets, bucket_option);

    // Sort LMS
    let lms_suffix_array = {
        let mut orders: Vec<usize> = vec![0; lms_indices.len()];
        let mut sort_order = 0usize;

        // Scan buckets
        for bucket in buckets.iter() {
            // S in backward
            for &index in bucket.s_typed.iter().rev() {
                if index > 0 {
                    match types[index - 1] {
                        Type::L => {
                            orders[lms_orders[index]] = sort_order;
                            sort_order += 1;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Calc LMS suffix array
        suffix_array(&orders, &IndexBucket { size: orders.len() })
    };

    // Clear all items from buckets
    for bucket_index in 0..buckets.len() {
        buckets[bucket_index].l_typed.clear();
        buckets[bucket_index].s_typed.clear();
    }

    // insert left-most S-typed indices into S-typed buckets in backward-backward order
    for &suffix_index in lms_suffix_array.iter().rev() {
        let index = lms_indices[suffix_index];
        let bucket_index = bucket_option.bucket_index(&data[index]);
        buckets[bucket_index].s_typed.push(index);
    }

    // Induced sort
    induced_sort(data, &types, &mut buckets, bucket_option);

    // Flat buckets
    buckets
        .into_iter()
        .flat_map(|bucket| {
            bucket
                .l_typed
                .into_iter()
                .chain(bucket.s_typed.into_iter().rev())
        })
        .collect()
}

struct Key {
    index: usize,
}

struct Value {}

pub struct DrainingSuffixArray<'a> {
    data: &'a [u8],
    tree: BTreeMap<Key, Value>,
}

impl<'a> DrainingSuffixArray<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            tree: BTreeMap::new(),
        }
    }

    pub fn drain(&mut self, size: usize) {
        self.data = &self.data[..size]
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let sa = suffix_array(&input.chars().collect::<Vec<_>>(), &CharBucket);
    println!("{:?}", sa);
}
