use crate::hkt::HKT2;

pub trait Functor<A, B>: HKT2<A, B> {
    fn fmap<F>(self, f: F) -> <Self as HKT2<A, B>>::Target
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
