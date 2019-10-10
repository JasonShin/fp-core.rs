use crate::apply::Apply;
use crate::pure::Pure;

pub trait Applicative<B>: Apply<B> + Pure<B> {}

impl<A, B> Applicative<B> for Option<A> {}
impl<A, B> Applicative<B> for Vec<A> {}

impl<A, B, E> Applicative<B> for Result<A, E> {}
