use crate::apply::Apply;
use crate::hkt::HKT;
use crate::pure::Pure;

pub trait Applicative<A, F, B>: Apply<F, B> + Pure<A>
where
    F: FnOnce(<Self as HKT<B>>::Current) -> B,
{
}

impl<A, F, B> Applicative<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
