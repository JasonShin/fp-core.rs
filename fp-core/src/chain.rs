use crate::hkt::HKT;

pub trait Chain<B>: HKT<B> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(Self::Current) -> Self::Target;
}

impl<A, B> Chain<B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(A) -> Self::Target,
    {
        self.and_then(f)
    }
}
