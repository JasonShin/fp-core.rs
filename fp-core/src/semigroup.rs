use std::ops::{Add, Mul};

pub trait Semigroup<M>: Add<M> {}
