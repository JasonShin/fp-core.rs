use fp_core::functor::Functor;
use fp_core::hkt::HKT;

// HKT 3
pub trait HKT3<A, B, C> {
    type Target2;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

// Apply
pub trait Apply<A, F, B>: Functor<A, B> + HKT3<A, F, B>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Target2) -> <Self as HKT<A, B>>::Target;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}

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
