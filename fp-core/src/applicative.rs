use crate::apply::Apply;
use crate::pure::Pure;

pub trait Applicative<B>: Apply<B> + Pure<B> {}

impl<A, B> Applicative<B> for Option<A> {}
