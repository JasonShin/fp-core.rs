use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn of(self) -> <Self as HKT<A>>::Source;
}

impl<A> Pure<A> for Option<A> {
    fn of(self) -> Self::Source {
        self
    }
}
