use std::{fmt::Debug, marker::PhantomData};

use super::suffix_array::{lcp_array, rank_array, suffix_array, BucketOption};

pub struct SuffixReference<'a, T: PartialEq + PartialOrd> {
    // data: &'a [T],
    suffix_array: Vec<usize>,
    rank_array: Vec<usize>,
    // lcp_array: Vec<usize>,
    back_array: Vec<Option<LcpBack>>,
    phantom: PhantomData<&'a T>,
}

#[derive(Clone, Copy, Debug)]
struct LcpHead {
    pub index: usize,
    pub lcp: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct LcpBack {
    pub index: usize,
    pub lcp: usize,
}

pub fn back_array(suffix_array: &[usize], lcp_array: &[usize]) -> Vec<Option<LcpBack>> {
    let mut back_array: Vec<Option<LcpBack>> = vec![None; suffix_array.len()];
    let mut heads: Vec<LcpHead> = Vec::new();

    for rank in 0..suffix_array.len() {
        let index = suffix_array[rank];
        let mut acc_lcp = usize::MAX;

        while let Some(head) = heads.last_mut() {
            head.lcp = head.lcp.min(acc_lcp);

            if index >= head.index {
                break;
            }

            back_array[head.index] = Some(LcpBack {
                index,
                lcp: head.lcp,
            });

            acc_lcp = head.lcp;

            heads.pop();
        }

        let next_lcp = lcp_array.get(rank + 1).copied().unwrap_or(0);
        heads.push(LcpHead {
            index,
            lcp: next_lcp,
        })
    }

    heads.clear();

    for rank in (0..suffix_array.len()).rev() {
        let index = suffix_array[rank];
        let mut acc_lcp = usize::MAX;

        while let Some(head) = heads.last_mut() {
            head.lcp = head.lcp.min(acc_lcp);

            if index >= head.index {
                break;
            }

            if back_array[head.index].map_or(true, |back| {
                (head.lcp > back.lcp) || ((head.lcp == back.lcp) && (index > back.index))
            }) {
                back_array[head.index] = Some(LcpBack {
                    index,
                    lcp: head.lcp,
                });
            }

            acc_lcp = head.lcp;

            heads.pop();
        }

        let next_lcp = lcp_array[rank];
        heads.push(LcpHead {
            index,
            lcp: next_lcp,
        })
    }

    back_array
}

impl<'a, T: PartialEq + PartialOrd + Debug> SuffixReference<'a, T> {
    pub fn from_data(data: &'a [T], bucket_option: &impl BucketOption<T>) -> Self {
        let suffix_array = suffix_array(data, bucket_option);
        let rank_array = rank_array(&suffix_array);
        let lcp_array = lcp_array(data, &suffix_array, &rank_array);
        let back_array = back_array(&suffix_array, &lcp_array);

        Self {
            // data,
            suffix_array,
            rank_array,
            // lcp_array,
            back_array,
            phantom: PhantomData,
        }
    }

    pub fn index(&self, rank: usize) -> usize {
        self.suffix_array[rank]
    }

    pub fn rank(&self, index: usize) -> usize {
        self.rank_array[index]
    }

    pub fn back(&self, index: usize) -> Option<LcpBack> {
        self.back_array.get(index).copied().unwrap_or(None)
    }
}
