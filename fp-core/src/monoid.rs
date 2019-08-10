use crate::empty::Empty;
use crate::semigroup::Semigroup;

pub trait Monoid: Empty + Semigroup {}
