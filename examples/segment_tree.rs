use azuki::core::{
    algebra::{Associative, BinaryOperable, WithIdentity},
    segment_tree::SegmentTree,
};

#[derive(Clone, Copy, Debug)]
struct Addable(usize);

impl BinaryOperable for Addable {
    fn operate(&self, other: &Self) -> Self {
        Addable(self.0 + other.0)
    }
}

impl Associative for Addable {}
impl WithIdentity for Addable {
    fn identity() -> Self {
        Addable(0)
    }
}

fn main() {
    let mut tree: SegmentTree<Addable> = SegmentTree::with_len(20);

    println!(
        "{:?} + {:?} + {:?} = {:?}",
        tree.get(3),
        tree.get(4),
        tree.get(5),
        tree.get(3..6)
    );

    tree.set(0, Addable(3));
    tree.set(1, Addable(1));
    tree.set(2, Addable(4));
    tree.set(3, Addable(1));
    tree.set(4, Addable(5));
    tree.set(5, Addable(9));
    tree.set(6, Addable(2));
    tree.set(7, Addable(6));
    tree.set(8, Addable(5));
    tree.set(9, Addable(3));
    tree.set(10, Addable(5));

    println!(
        "+{:?} = {:?}",
        (3..6).map(|i| tree.get(i)).collect::<Vec<_>>(),
        tree.get(3..6)
    );

    println!(
        "+{:?} = {:?}",
        (1..19).map(|i| tree.get(i)).collect::<Vec<_>>(),
        tree.get(1..19)
    );
}
