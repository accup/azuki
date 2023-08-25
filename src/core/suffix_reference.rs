use std::marker::PhantomData;

use super::suffix_array::{lcp_array, rank_array, suffix_array, BucketOption};

pub struct SuffixReference<'a, T: PartialEq + PartialOrd> {
    // data: &'a [T],
    // suffix_array: Vec<usize>,
    // rank_array: Vec<usize>,
    // lcp_array: Vec<usize>,
    back_array: Vec<Option<LcpHead>>,
    phantom: PhantomData<&'a T>,
}

#[derive(Clone, Copy)]
pub struct LcpHead {
    pub index: usize,
    pub lcp: usize,
}

fn back_array(suffix_array: &[usize], lcp_array: &[usize]) -> Vec<Option<LcpHead>> {
    let mut back_array: Vec<Option<LcpHead>> = vec![None; suffix_array.len()];

    let mut heads: Vec<LcpHead> = Vec::new();
    for rank in 0..suffix_array.len() {
        let index = suffix_array[rank];
        let mut lcp = usize::MAX;

        while let Some(head) = heads.last_mut() {
            head.lcp = head.lcp.min(lcp);

            if index >= head.index {
                break;
            }

            back_array[head.index] = Some(LcpHead {
                index,
                lcp: head.lcp,
            });

            lcp = head.lcp;

            heads.pop();
        }

        heads.push(LcpHead {
            index,
            lcp: if rank + 1 < lcp_array.len() {
                lcp_array[rank + 1]
            } else {
                0
            },
        })
    }

    heads.clear();

    for rank in (0..suffix_array.len()).rev() {
        let index = suffix_array[rank];
        let mut lcp = usize::MAX;

        while let Some(head) = heads.last_mut() {
            head.lcp = head.lcp.min(lcp);

            if index >= head.index {
                break;
            }

            if back_array[head.index].map_or(true, |back| {
                (head.lcp > back.lcp) || ((head.lcp == back.lcp) && (index > back.index))
            }) {
                back_array[head.index] = Some(LcpHead {
                    index,
                    lcp: head.lcp,
                });
            }

            lcp = head.lcp;

            heads.pop();
        }

        heads.push(LcpHead {
            index,
            lcp: lcp_array[rank],
        })
    }

    back_array
}

impl<'a, T: PartialEq + PartialOrd> SuffixReference<'a, T> {
    pub fn from_data(data: &'a [T], bucket_option: &impl BucketOption<T>) -> Self {
        let suffix_array = suffix_array(data, bucket_option);
        let rank_array = rank_array(&suffix_array);
        let lcp_array = lcp_array(data, &suffix_array, &rank_array);
        let back_array = back_array(&suffix_array, &lcp_array);

        Self {
            // data,
            // suffix_array,
            // rank_array,
            // lcp_array,
            back_array,
            phantom: PhantomData,
        }
    }

    pub fn back(&self, index: usize) -> Option<LcpHead> {
        self.back_array.get(index).copied().unwrap_or(None)
    }
}
