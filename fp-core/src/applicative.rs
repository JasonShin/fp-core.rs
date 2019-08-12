use crate::apply::Apply;
use crate::pure::Pure;

pub trait Applicative<A, F, B>: Apply<A, B> + Pure<A>
where
    F: FnOnce(A) -> B,
{
}

impl<A, F, B> Applicative<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
