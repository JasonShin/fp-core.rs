use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn of(c: Self::Current) -> Self::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(a: A) -> Self::Target {
        Some(a)
    }
}
