use crate::hkt::HKT;

pub trait Functor<A, B>: HKT<A, B> {
    fn fmap<F>(self, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A) -> B;
}

impl<A, B> Functor<A, B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}
