use std::ops::{Add};

pub trait Semigroup<M>: Add<M> {}
