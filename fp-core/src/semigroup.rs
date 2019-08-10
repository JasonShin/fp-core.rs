use std::ops::Add;

pub trait Semigroup<T> {
    fn combine(self, other: T) -> T;
}

impl<T, I> Semigroup<T> for I
where
    I: Add<T, Output = T>,
    Self: Into<T>,
{
    fn combine(self, other: T) -> T {
        self.add(other)
    }
}
