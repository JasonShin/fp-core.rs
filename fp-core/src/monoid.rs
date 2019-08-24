use crate::empty::Empty;
use crate::semigroup::Semigroup;
use std::ops::Add;

pub trait Monoid: Empty + Semigroup {}

// impl<I> Monoid for I where I: Add<I, Output = I> + From<i32> {}
