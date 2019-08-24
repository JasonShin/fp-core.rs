use crate::hkt::HKT;
use crate::monoid::Monoid;

pub trait Foldable<A, B>: HKT<A, B> {
    fn reduce<F>(b: B, ba: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(B, A) -> (B, B);

    fn fold_map<M, N, F>(m: M, fa: F) -> M
    where
        M: Monoid<N>,
        F: FnOnce(<Self as HKT<A, B>>::Source) -> M;

    fn reduce_right<F>(b: B, f: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(A, B) -> (B, B);
}
