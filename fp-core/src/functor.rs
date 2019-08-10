use crate::hkt::HKT;

pub trait Functor<B>: HKT<B> {
    fn fmap<F>(self, f: F) -> <Self as HKT<B>>::Target
    where
        F: FnOnce(<Self as HKT<B>>::Current) -> B;
}

impl<A, B> Functor<B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(Self::Current) -> B,
    {
        self.map(f)
    }
}
