use crate::functor::Functor;
use crate::hkt::HKT2;

pub trait Extend<A, B>: Functor<A, B> + Sized {
    fn extend<W>(self, f: W) -> <Self as HKT2<A, B>>::Target
    where
        W: FnOnce(Self) -> B;
}

impl<A, B> Extend<A, B> for Option<A> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Some(x)))
    }
}
