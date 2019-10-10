use crate::hkt::HKT;

pub trait Functor<B>: HKT<B> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: FnOnce(Self::Current) -> B;
}

impl<A, B> Functor<B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        // A is Self::Current
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B> Functor<B> for Vec<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        // A is Self::Current
        F: FnOnce(A) -> B,
    {
        self.iter().map(f).collect()
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        // A is Self::Current
        F: FnOnce(A) -> B,
    {
        self.map(f)
    }
}
