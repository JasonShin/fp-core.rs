use fp_core::apply::Apply;
use fp_core::functor::Functor;
use fp_core::hkt::{HKT, HKT3};

// Apply

// Pure
pub trait Pure<A>: HKT<A, A> {
    fn of(self) -> <Self as HKT<A, A>>::Target;
}

impl<A> Pure<A> for Option<A> {
    fn of(self) -> Self::Target {
        self
    }
}

// Applicative
pub trait Applicative<A, F, B>: Apply<A, F, B> + Pure<A>
where
    F: FnOnce(A) -> B,
{
}

impl<A, F, B> Applicative<A, F, B> for Option<A> where F: FnOnce(A) -> B {}

#[test]
fn applicative_example() {
    let x = Option::of(Some(1)).ap(Some(|x| x + 1));
    assert_eq!(x, Some(2));
}
