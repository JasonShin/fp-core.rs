use crate::applicative::Applicative;
use crate::chain::Chain;

pub trait Monad<B>: Chain<B> + Applicative<B> {}

impl<A> Monad<A> for Option<A> {}

impl<A, E> Monad<A> for Result<A, E> {}
