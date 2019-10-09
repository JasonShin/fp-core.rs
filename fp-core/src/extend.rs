use crate::functor::Functor;
use crate::hkt::HKT;

pub trait Extend<B>: Functor<B> + Sized {
    fn extend<W>(self, f: W) -> <Self as HKT<B>>::Target
    where
        W: FnOnce(Self) -> B;
}

impl<A, B> Extend<B> for Option<A> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Some(x)))
    }
}

impl<A, B, E> Extend<B> for Result<A, E> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Ok(x)))
    }
}
