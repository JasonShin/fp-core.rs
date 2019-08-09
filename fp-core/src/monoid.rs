use crate::applicative::Applicative;
use crate::empty::Empty;
use std::ops::Add;

pub trait Monoid<M>: Empty<M> + Add<M> {}
