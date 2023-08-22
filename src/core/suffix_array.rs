pub trait BucketOption<T> {
    fn size(&self) -> usize;
    fn bucket_index(&self, value: &T) -> usize;
}

pub struct U8Bucket;

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

fn induced_sort<T, B: BucketOption<T>>(
    data: &[T],
    types: &[Type],
    buckets: &mut Vec<SuffixArrayBucket>,
    bucket_option: &B,
) {
    // insert the last L-typed item
    if data.len() > 0 {
        let index = data.len() - 1;

        if let Type::L = types[index] {
            let bucket_index = bucket_option.bucket_index(&data[index]);
            buckets[bucket_index].l_typed.push(index);
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
                    if let Type::L = types[index - 1] {
                        let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                        buckets[bucket_index].l_typed.push(index - 1);
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
                    if let Type::L = types[index - 1] {
                        let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                        buckets[bucket_index].l_typed.push(index - 1);
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
                    if let Type::S = types[index - 1] {
                        let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                        buckets[bucket_index].s_typed.push(index - 1);
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
                    if let Type::S = types[index - 1] {
                        let bucket_index = bucket_option.bucket_index(&data[index - 1]);
                        buckets[bucket_index].s_typed.push(index - 1);
                    }
                }

                rev_l_index += 1;
            }
        }
    }
}

pub fn suffix_array<T, B: BucketOption<T>>(data: &[T], bucket_option: &B) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let mut types = vec![Type::L; data.len()];

    for index in (1..data.len()).rev() {
        let bucket_index0 = bucket_option.bucket_index(&data[index - 1]);
        let bucket_index1 = bucket_option.bucket_index(&data[index]);
        types[index - 1] = if bucket_index0 == bucket_index1 {
            types[index]
        } else if bucket_index0 < bucket_index1 {
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
        let Type::L = types[index - 1] else { continue };
        let Type::S = types[index] else { continue };

        lms_orders[index] = lms_indices.len();
        lms_indices.push(index);
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
        let mut lms_ranks: Vec<usize> = vec![0; lms_indices.len()];
        let mut lms_rank = 0usize;

        // Scan buckets
        for bucket in buckets.iter() {
            // S in backward-backward
            for &index in bucket.s_typed.iter().rev() {
                if index <= 0 {
                    continue;
                }

                let Type::L = types[index - 1] else { continue };

                lms_ranks[lms_orders[index]] = lms_rank;
                lms_rank += 1;
            }
        }

        // Calc LMS suffix array
        suffix_array(
            &lms_ranks,
            &IndexBucket {
                size: lms_ranks.len(),
            },
        )
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

pub fn rank_array(suffix_array: &[usize]) -> Vec<usize> {
    let mut rank_array = vec![0usize; suffix_array.len()];

    for (rank, &index) in suffix_array.iter().enumerate() {
        rank_array[index] = rank;
    }

    rank_array
}

pub fn lcp_array<T: PartialEq + PartialOrd>(
    data: &[T],
    suffix_array: &[usize],
    rank_array: &[usize],
) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let mut lcp_array = vec![0usize; data.len()];
    let mut lcp = 0;

    for index1 in 0..data.len() {
        let rank1 = rank_array[index1];
        let rank0 = if rank1 > 0 {
            rank1 - 1
        } else {
            lcp = 0;
            continue;
        };
        let index0 = suffix_array[rank0];

        while index0 + lcp < data.len()
            && index1 + lcp < data.len()
            && &data[index0 + lcp] == &data[index1 + lcp]
        {
            lcp += 1;
        }

        lcp_array[rank1] = lcp;
        lcp = lcp.saturating_sub(1);
    }

    lcp_array
}
