use crate::applicative::Applicative;
use crate::empty::Empty;
use crate::semigroup::Semigroup;

pub trait Monoid<M>: Empty<M> + Semigroup<M> {}
