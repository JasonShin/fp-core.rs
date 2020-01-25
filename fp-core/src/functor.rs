use crate::hkt::HKT;

pub trait Functor<B>: HKT<B> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: Fn(Self::Current) -> B;
}

impl<A, B> Functor<B> for Option<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        // A is Self::Current
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<A, B> Functor<B> for Vec<A> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        F: Fn(<Self as HKT<B>>::Current) -> B,
    {
        self.into_iter().map(f).collect::<Vec<B>>()
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    fn fmap<F>(self, f: F) -> Self::Target
    where
        // A is Self::Current
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}
