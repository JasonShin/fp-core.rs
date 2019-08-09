use std::ops::{Add, Sub};

pub trait Semigroup<M>: Add<M> + Sub<M> {}
