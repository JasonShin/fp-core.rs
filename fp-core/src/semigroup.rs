use std::ops::Add;

pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

impl Semigroup for i32 {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}

impl Semigroup for i64 {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}

impl<T: Clone> Semigroup for Vec<T> {
    fn combine(self, other: Self) -> Self {
        let mut concat = self.to_vec();
        concat.extend_from_slice(&other);
        concat
    }
}
