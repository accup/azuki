use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

use super::algebra::{Associative, BinaryOperable, WithIdentity};

pub struct SegmentTree<T>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    len: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    pub fn with_len(len: usize) -> Self {
        Self {
            len,
            data: vec![T::identity(); len.next_power_of_two() << 1],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn cursor(&self, index: usize) -> usize {
        self.start_cursor() + index
    }

    fn start_cursor(&self) -> usize {
        self.data.len() >> 1
    }

    pub fn set(&mut self, index: usize, value: T) {
        let mut cursor = self.cursor(index);
        self.data[cursor] = value;

        while cursor > 1 {
            let lhs = &self.data[cursor & (!1)];
            let rhs = &self.data[cursor | 1];

            cursor >>= 1;
            self.data[cursor] = lhs.operate(rhs);
        }
    }

    pub fn get<Idx: SegmentTreeIndex<T>>(&self, index: Idx) -> T {
        index.get(&self)
    }
}

pub trait SegmentTreeIndex<T>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T;
}

impl<T> SegmentTreeIndex<T> for usize
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T {
        tree.data[tree.cursor(self)].clone()
    }
}

impl<T> SegmentTreeIndex<T> for RangeFull
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T {
        tree.data[1].clone()
    }
}

impl<T> SegmentTreeIndex<T> for RangeFrom<usize>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T {
        (self.start..tree.len()).get(tree)
    }
}

impl<T> SegmentTreeIndex<T> for RangeTo<usize>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T {
        (0..self.end).get(tree)
    }
}

impl<T> SegmentTreeIndex<T> for Range<usize>
where
    T: Clone + BinaryOperable + Associative + WithIdentity,
{
    fn get(self, tree: &SegmentTree<T>) -> T {
        let mut l_cursor = tree.cursor(self.start) - 1;
        let mut r_cursor = tree.cursor(self.end);
        let mut l_acc = T::identity();
        let mut r_acc = T::identity();

        while l_cursor + 1 < r_cursor {
            if l_cursor & 1 == 0 {
                l_acc = l_acc.operate(&tree.data[l_cursor + 1]);
            }
            if r_cursor & 1 == 1 {
                r_acc = r_acc.operate(&tree.data[r_cursor - 1]);
            }

            l_cursor >>= 1;
            r_cursor >>= 1;
        }

        l_acc.operate(&r_acc)
    }
}
