use crate::hkt::HKT;

pub trait Pure<A>: HKT<A, A> {
    fn of(self) -> <Self as HKT<A, A>>::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(self) -> Self::Target {
        self
    }
}
