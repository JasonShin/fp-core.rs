use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn of(c: A) -> Self::Target;
}

impl<A, B> Pure<A> for Option<B> {
    fn of(a: A) -> <Self as HKT<A>>::Target {
        Some(a)
    }
}

impl<A, B> Pure<A> for Vec<B> {
    fn of(a: A) -> Self::Target {
        vec![a]
    }
}

impl<A, B, E> Pure<A> for Result<B, E> {
    fn of(a: A) -> <Self as HKT<A>>::Target {
        Ok(a)
    }
}
