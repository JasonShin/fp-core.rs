use crate::applicative::Applicative;
use crate::chain::Chain;

pub trait Monad<A, F, B>: Chain<A, B> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}

impl<A, F, B> Monad<A, F, B> for Option<A> where F: FnOnce(A) -> B {}
