use crate::hkt::HKT;

pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> <Self as HKT<A, B>>::Target,
    {
        self.and_then(f)
    }
}
