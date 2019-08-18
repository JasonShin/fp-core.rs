use crate::empty::Empty;
use crate::semigroup::Semigroup;
use std::ops::Add;

pub trait Monoid: Empty + Semigroup {}
