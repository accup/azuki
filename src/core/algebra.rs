pub trait BinaryOperable {
    fn operate(&self, other: &Self) -> Self;
}

pub trait Associative
where
    Self: BinaryOperable,
{
}

pub trait WithIdentity
where
    Self: BinaryOperable,
{
    fn identity() -> Self;
}
