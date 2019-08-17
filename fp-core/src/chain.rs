use crate::hkt::HKT;

pub trait Chain<B>: HKT<B> {
    fn chain<F>(self, f: F) -> <Self as HKT<B>>::Target
    where
        F: FnOnce(<Self as HKT<B>>::Current) -> <Self as HKT<B>>::Target;
}

impl<A, B> Chain<B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> <Self as HKT<B>>::Target,
    {
        self.and_then(f)
    }
}
