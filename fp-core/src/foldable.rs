use crate::hkt::HKT;
use crate::monoid::Monoid;

use std::fmt::Debug;

// Cheating: all HKT instances exist for any B,
// so HKT<B> here isn't about Self<B> having any meaning,
// it's about folding on some Self<A> -- the HKT lets us
// have an A to speak of.
pub trait Foldable<B>: HKT<B> + Sized {
    fn reduce<F>(self, b: B, ba: F) -> B
    where
        F: Fn(B, &<Self as HKT<B>>::Current) -> B;

    fn reduce_right<F>(self, b: B, f: F) -> B
    where
        F: Fn(&<Self as HKT<B>>::Current, B) -> B;
}

// Biggest hardship with trying to put this into the above:
// we cannot have B constrained to be a Monoid, so having
// a default implementation becomes impossible. That said,
// having this as a separate function might make more sense
// (in particular, it might be easier to implement Foldable for
// rust containers as above and not have to worry about our Monoid
// until "later" -- when this function is handy).
pub fn fold_map<M, C, F>(container: C, mapper: F) -> M
where
    M: Monoid,
    C: Foldable<M>,
    F: Fn(&<C as HKT<M>>::Current) -> M,
{
    container.reduce(M::empty(), |acc, curr| acc.combine(mapper(curr)))
}

impl<A, B> Foldable<B> for Vec<A> {
    fn reduce<F>(self, b: B, fa: F) -> B
    where
        F: Fn(B, &A) -> B,
    {
        self.iter().fold(b, fa)
    }

    // TODO: make sure this is correct.
    fn reduce_right<F>(self, b: B, fa: F) -> B
    where
        F: Fn(&A, B) -> B,
    {
        self.iter().rev().fold(b, |x, y| fa(y, x))
    }
}

impl<A, B> Foldable<B> for Option<A> {
    fn reduce<F>(self, b: B, fa: F) -> B
    where
        F: Fn(B, &A) -> B,
    {
        self.as_ref().and_then(|v| Option::Some(fa(b, v))).unwrap()
    }

    fn reduce_right<F>(self, b: B, fa: F) -> B
    where
        F: Fn(&A, B) -> B,
    {
        self.as_ref().and_then(|v| Option::Some(fa(v, b))).unwrap()
    }
}

impl<A, B, E: Debug> Foldable<B> for Result<A, E> {
    fn reduce<F>(self, b: B, fa: F) -> B
    where
        F: Fn(B, &A) -> B,
    {
        self.as_ref().and_then(|v| Result::Ok(fa(b, v))).unwrap()
    }

    fn reduce_right<F>(self, b: B, fa: F) -> B
    where
        F: Fn(&A, B) -> B,
    {
        self.as_ref().and_then(|v| Result::Ok(fa(v, b))).unwrap()
    }
}
