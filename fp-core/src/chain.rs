use crate::hkt::HKT2;

pub trait Chain<A, B>: HKT2<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT2<A, B>>::Target
    where
        F: FnOnce(A) -> <Self as HKT2<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> <Self as HKT2<A, B>>::Target,
    {
        self.and_then(f)
    }
}
