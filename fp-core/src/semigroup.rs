use std::ops::Add;

pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

impl<I> Semigroup for I
where
    I: Add<I>,
{
    fn combine(self, other: I) -> I {
        self.add(other)
    }
}
