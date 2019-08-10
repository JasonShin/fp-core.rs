use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn of(c: <Self as HKT<A>>::Current) -> <Self as HKT<A>>::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(a: A) -> Self::Target {
        Some(a)
    }
}
