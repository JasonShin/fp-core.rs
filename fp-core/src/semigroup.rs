use im::vector::*;
use std::ops::Add;
use self::im::vector::proptest::vector;
use std::convert::TryInto;

#[derive(Clone)]
pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

impl<I> Semigroup for I
where
    I: Add<I, Output = I>,
    I: Iterator
{
    fn combine(self, other: I) -> I {

        self.add(other)
    }
}

/*
impl Semigroup for i32 {
    fn combine(self, other: Self) -> Self {
       self + other
    }
}

impl<A, F> Semigroup for F
where
    F: Iterator<A> {
    fn combine(&mut self, &mut other: Self) -> Self {
        self.fold()
    }
}
*/
