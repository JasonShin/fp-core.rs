use crate::applicative::Applicative;
use crate::chain::Chain;
use crate::hkt::HKT;

pub trait Monad<A, F, B>: Chain<B> + Applicative<A, F, B>
where
    F: FnOnce(<Self as HKT<B>>::Current) -> B,
{
}

impl<A, F, B> Monad<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
