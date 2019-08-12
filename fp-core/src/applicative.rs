use crate::apply::Apply;
use crate::pure::Pure;

pub trait Applicative<A, B>: Apply<B> + Pure<A> {}

impl<A, B> Applicative<A, B> for Option<A> {}
