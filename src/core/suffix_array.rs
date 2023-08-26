use std::marker::PhantomData;

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

pub struct IndexBucket {
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
pub struct SuffixArrayBucket<'a, T, Bucket: BucketOption<T>> {
    bucket_option: &'a Bucket,
    indices: Vec<usize>,
    bins: Vec<BucketBin>,
    phantom: PhantomData<T>,
}

#[derive(Clone)]
struct BucketBin {
    l_start: usize,
    l_count: usize,
    s_stop: usize,
    s_count: usize,
}

impl<'a, T, Bucket: BucketOption<T>> SuffixArrayBucket<'a, T, Bucket> {
    pub fn new(data: &[T], types: &[SuffixType], bucket_option: &'a Bucket) -> Self {
        let mut bins = vec![
            BucketBin {
                l_start: 0,
                l_count: 0,
                s_stop: 0,
                s_count: 0
            };
            bucket_option.size()
        ];

        for (value, suffix_type) in data.iter().zip(types.iter()) {
            let bucket_index = bucket_option.bucket_index(value);

            match suffix_type {
                SuffixType::L => {
                    bins[bucket_index].l_start += 1;
                }
                SuffixType::S => {
                    bins[bucket_index].s_stop += 1;
                }
            }
        }

        // <--- bin0 ---><------ bin1 ------>
        // 0000000000000011111111111111111111
        // ^            ^^                  ^
        // l_start      ^l_start            ^
        //          s_end               s_end

        let first_bin = &mut bins[0];
        first_bin.s_stop += first_bin.l_start;
        first_bin.l_start = 0;

        for index in 1..bins.len() {
            bins[index].s_stop += bins[index].l_start + bins[index - 1].s_stop;
            bins[index].l_start = bins[index - 1].s_stop;
        }

        Self {
            bucket_option,
            indices: vec![0; data.len()],
            bins,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn num_bins(&self) -> usize {
        self.bins.len()
    }

    pub fn len_l_bin(&self, bin: usize) -> usize {
        self.bins[bin].l_count
    }

    pub fn len_s_bin(&self, bin: usize) -> usize {
        self.bins[bin].s_count
    }

    pub fn l_index_by_rank(&self, bin: usize, rank: usize) -> usize {
        let bin = &self.bins[bin];
        self.indices[bin.l_start + rank]
    }

    pub fn l_index_by_rev_rank(&self, bin: usize, rev_rank: usize) -> usize {
        let bin = &self.bins[bin];
        self.indices[(bin.l_start + bin.l_count - 1) - rev_rank]
    }

    pub fn s_index_by_rank(&self, bin: usize, rank: usize) -> usize {
        let bin = &self.bins[bin];
        self.indices[(bin.s_stop - bin.s_count) + rank]
    }

    pub fn s_index_by_rev_rank(&self, bin: usize, rev_rank: usize) -> usize {
        let bin = &self.bins[bin];
        self.indices[(bin.s_stop - 1) - rev_rank]
    }

    pub fn iter_bins(&self) -> BothBucketIterator<'_, 'a, T, Bucket> {
        BothBucketIterator {
            bucket: &self,
            next_bin: 0,
            next_type: SuffixType::L,
        }
    }

    pub fn iter_l_bins(&self) -> TypeLBucketIterator<'_, 'a, T, Bucket> {
        TypeLBucketIterator {
            bucket: &self,
            next_bin: 0,
        }
    }

    pub fn iter_s_bins(&self) -> TypeSBucketIterator<'_, 'a, T, Bucket> {
        TypeSBucketIterator {
            bucket: &self,
            next_bin: 0,
        }
    }

    pub fn push(&mut self, index: usize, value: &T, suffix_type: SuffixType) {
        let bucket_index = self.bucket_option.bucket_index(value);
        let bin = &mut self.bins[bucket_index];

        match suffix_type {
            SuffixType::L => {
                self.indices[bin.l_start + bin.l_count] = index;
                bin.l_count += 1;
            }
            SuffixType::S => {
                self.indices[(bin.s_stop - 1) - bin.s_count] = index;
                bin.s_count += 1;
            }
        }
    }

    pub fn clear(&mut self, bin: usize, suffix_type: SuffixType) {
        let bin = &mut self.bins[bin];

        match suffix_type {
            SuffixType::L => {
                bin.l_count = 0;
            }
            SuffixType::S => {
                bin.s_count = 0;
            }
        }
    }

    pub fn clear_both(&mut self, bin: usize) {
        let bin = &mut self.bins[bin];
        bin.l_count = 0;
        bin.s_count = 0;
    }

    pub fn clear_all(&mut self) {
        for bin in self.bins.iter_mut() {
            bin.l_count = 0;
            bin.s_count = 0;
        }
    }
}

impl<'a, T, B: BucketOption<T>> IntoIterator for SuffixArrayBucket<'a, T, B> {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.indices.into_iter()
    }
}

pub struct BothBucketIterator<'b, 'a, T, B: BucketOption<T>> {
    bucket: &'b SuffixArrayBucket<'a, T, B>,
    next_bin: usize,
    next_type: SuffixType,
}

impl<'b, 'a, T, B: BucketOption<T>> Iterator for BothBucketIterator<'b, 'a, T, B> {
    type Item = &'b [usize];

    fn next(&mut self) -> Option<Self::Item> {
        let Some(bin) = self.bucket.bins.get(self.next_bin) else { return None };

        Some(match self.next_type {
            SuffixType::L => {
                let slice = &self.bucket.indices[bin.l_start..(bin.l_start + bin.l_count)];
                self.next_type = SuffixType::S;
                slice
            }
            SuffixType::S => {
                let slice = &self.bucket.indices[(bin.s_stop - bin.s_count)..bin.s_stop];
                self.next_bin += 1;
                self.next_type = SuffixType::L;
                slice
            }
        })
    }
}

pub struct TypeLBucketIterator<'b, 'a, T, B: BucketOption<T>> {
    bucket: &'b SuffixArrayBucket<'a, T, B>,
    next_bin: usize,
}

impl<'b, 'a, T, B: BucketOption<T>> Iterator for TypeLBucketIterator<'b, 'a, T, B> {
    type Item = &'b [usize];

    fn next(&mut self) -> Option<Self::Item> {
        let Some(bin) = self.bucket.bins.get(self.next_bin) else { return None };

        Some({
            let slice = &self.bucket.indices[bin.l_start..(bin.l_start + bin.l_count)];
            self.next_bin += 1;
            slice
        })
    }
}

pub struct TypeSBucketIterator<'b, 'a, T, B: BucketOption<T>> {
    bucket: &'b SuffixArrayBucket<'a, T, B>,
    next_bin: usize,
}

impl<'b, 'a, T, B: BucketOption<T>> Iterator for TypeSBucketIterator<'b, 'a, T, B> {
    type Item = &'b [usize];

    fn next(&mut self) -> Option<Self::Item> {
        let Some(bin) = self.bucket.bins.get(self.next_bin) else { return None };

        Some({
            let slice = &self.bucket.indices[(bin.s_stop - bin.s_count)..bin.s_stop];
            self.next_bin += 1;
            slice
        })
    }
}

#[derive(Copy, Clone)]
pub enum SuffixType {
    S,
    L,
}

fn induced_sort<T, B: BucketOption<T>>(
    data: &[T],
    types: &[SuffixType],
    bucket: &mut SuffixArrayBucket<T, B>,
) {
    // insert the last L-typed item
    if data.len() > 0 {
        let index = data.len() - 1;

        if let SuffixType::L = types[index] {
            bucket.push(index, &data[index], SuffixType::L);
        }
    }

    // insert all other L-typed items
    for bin in 0..bucket.num_bins() {
        {
            let mut rank = 0usize;

            while rank < bucket.len_l_bin(bin) {
                let index = bucket.l_index_by_rank(bin, rank);

                if index > 0 {
                    if let SuffixType::L = types[index - 1] {
                        bucket.push(index - 1, &data[index - 1], SuffixType::L);
                    }
                }

                rank += 1;
            }
        }
        {
            let mut rank = 0usize;

            while rank < bucket.len_s_bin(bin) {
                let index = bucket.s_index_by_rank(bin, rank);

                if index > 0 {
                    if let SuffixType::L = types[index - 1] {
                        bucket.push(index - 1, &data[index - 1], SuffixType::L);
                    }
                }

                rank += 1;
            }
        }
    }

    // Clear S-typed items from buckets
    for bin in 0..bucket.num_bins() {
        bucket.clear(bin, SuffixType::S);
    }

    // insert all S-typed items
    for bin in (0..bucket.num_bins()).rev() {
        {
            let mut rev_rank = 0usize;

            while rev_rank < bucket.len_s_bin(bin) {
                let index = bucket.s_index_by_rev_rank(bin, rev_rank);

                if index > 0 {
                    if let SuffixType::S = types[index - 1] {
                        bucket.push(index - 1, &data[index - 1], SuffixType::S);
                    }
                }

                rev_rank += 1;
            }
        }
        {
            let mut rev_rank = 0usize;

            while rev_rank < bucket.len_l_bin(bin) {
                let index = bucket.l_index_by_rev_rank(bin, rev_rank);

                if index > 0 {
                    if let SuffixType::S = types[index - 1] {
                        bucket.push(index - 1, &data[index - 1], SuffixType::S);
                    }
                }

                rev_rank += 1;
            }
        }
    }
}

pub fn suffix_array<T, B: BucketOption<T>>(data: &[T], bucket_option: &B) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }

    let mut types = vec![SuffixType::L; data.len()];

    for index in (1..data.len()).rev() {
        let bucket_index0 = bucket_option.bucket_index(&data[index - 1]);
        let bucket_index1 = bucket_option.bucket_index(&data[index]);
        types[index - 1] = if bucket_index0 == bucket_index1 {
            types[index]
        } else if bucket_index0 < bucket_index1 {
            SuffixType::S
        } else {
            SuffixType::L
        };
    }

    let mut lms_orders: Vec<usize> = vec![0; data.len()];
    let mut lms_ranges: Vec<(usize, usize)> = Vec::new();

    // collect left-most S-typed indices
    for index in 1..data.len() {
        let SuffixType::L = types[index - 1] else { continue };
        let SuffixType::S = types[index] else { continue };

        if let Some(lms_range) = lms_ranges.last_mut() {
            lms_range.1 = index + 1;
        }

        lms_orders[index] = lms_ranges.len();
        lms_ranges.push((index, data.len() + 1));
    }

    let mut bucket = SuffixArrayBucket::new(data, &types, bucket_option);

    // insert left-most S-typed indices into S-typed buckets
    for &(index, ..) in lms_ranges.iter() {
        bucket.push(index, &data[index], SuffixType::S);
    }

    // Induced sort
    induced_sort(data, &types, &mut bucket);

    // Sort LMS
    let lms_suffix_array = {
        let mut lms_ranks: Vec<usize> = vec![0; lms_ranges.len()];
        let mut lms_rank = 0usize;
        let mut last_lms_order: Option<usize> = None;

        // Scan buckets
        for bin in bucket.iter_s_bins() {
            for &index in bin {
                if index <= 0 {
                    continue;
                }

                let SuffixType::L = types[index - 1] else { continue };

                let lms_order = lms_orders[index];
                let lms_range = &lms_ranges[lms_order];

                if let Some(last_order) = last_lms_order {
                    let last_lms_range = lms_ranges[last_order];

                    let lms_len = lms_range.1 - lms_range.0;
                    let last_lms_len = last_lms_range.1 - last_lms_range.0;

                    let is_same = (lms_len == last_lms_len) && {
                        (0..lms_len).all(|i| {
                            let index = lms_range.0 + i;
                            let last_index = last_lms_range.0 + i;

                            // terminal character
                            if (index >= data.len() || last_index >= data.len())
                                && (index != last_index)
                            {
                                false
                            } else {
                                let class = bucket_option.bucket_index(&data[index]);
                                let last_class = bucket_option.bucket_index(&data[last_index]);
                                class == last_class
                            }
                        })
                    };

                    if !is_same {
                        lms_rank += 1;
                    }
                }

                lms_ranks[lms_order] = lms_rank;
                last_lms_order = Some(lms_order);
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
    bucket.clear_all();

    // insert left-most S-typed indices into S-typed buckets in backward-backward order
    for &suffix_index in lms_suffix_array.iter().rev() {
        let (index, ..) = lms_ranges[suffix_index];
        bucket.push(index, &data[index], SuffixType::S);
    }

    // Induced sort
    induced_sort(data, &types, &mut bucket);

    // Flat buckets
    bucket.into_iter().collect()
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

    let mut lcp_array = vec![0; data.len()];
    let mut lcp = 0;

    for index0 in 0..data.len() {
        let rank0 = rank_array[index0];
        let rank1 = if rank0 + 1 < data.len() {
            rank0 + 1
        } else {
            lcp = 0;
            continue;
        };
        let index1 = suffix_array[rank1];

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
