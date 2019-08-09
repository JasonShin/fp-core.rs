use crate::hkt::HKT;
use crate::monoid::Monoid;

pub trait Foldable<A, B>: HKT<A, B> {
    fn reduce<F>(b: B, ba: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(B, A) -> (B, B);
}
