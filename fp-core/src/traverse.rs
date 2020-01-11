use crate::applicative::Applicative;
use crate::foldable::Foldable;
use crate::functor::Functor;
use crate::hkt::{HktInHkt, HKT};
use crate::pure::Pure;

use std::fmt::Debug;

// Re-statement of the Haskell definition
// (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
pub trait Traversable<A, B>: Functor<B> + Foldable<B> {
    // A is the source type, F the functor, and AFB a natural transformation
    // from A into F<B>. Pursuant to Cats in Scala (https://www.scala-exercises.org/cats/traverse),
    // A would be a User, F a future, and Self would be a list. AFB would be the sort of "user
    // promise" function in the example.
    fn traverse<F, AFB>(&self, traverser: Box<AFB>) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B> + Applicative<<Self as HKT<B>>::Target>,
        AFB: Fn(A) -> <F as HKT<B>>::Target;
}

// Re-statement of the Haskell function
// (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
pub fn sequenceA<T, F, A>(wrong_order: HktInHkt<T, F, A>) -> HktInHkt<F, T, A>
where
    T: Traversable<A, A> + HKT<<F as HKT<A>>::Target>,
    F: Applicative<A> + HKT<<T as HKT<A>>::Target>,
{
    wrong_order.traverse(|x| x)
}

impl<A, B> Traversable<A, B> for Option<A> {
    fn traverse<F, AFB>(&self, traverser: Box<AFB>) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B> + Applicative<Option<B>>,
        AFB: Fn(A) -> <F as HKT<B>>::Target,
    {
        match &self {
            Option::None => <F as Pure<Option<B>>>::of(Option::None),
            Option::Some(a) => F::of(Option::Some(traverser(*a))),
        }
    }
}

impl<A, B> Traversable<A, B> for Vec<A> {
    fn traverse<F, AFB>(&self, traverser: Box<AFB>) -> HktInHkt<F, Self, B>
    where
        F: Applicative<B> + Applicative<Vec<B>>,
        AFB: Fn(A) -> <F as HKT<B>>::Target,
    {
        // Interestingly only uses that Self is a monoid.
        let acc = F::of(vec![]);
        for item in self {
            let t = traverser(*item);
            acc = acc.ap(|acc_list| acc_list + vec![t]);
        }
        return acc;
    }
}

impl<A, B, E: Debug> Traversable<A, B> for Result<A, E> {
    fn traverse<F, AFB>(&self, traverser: Box<AFB>) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B> + Applicative<Result<B, E>>,
        AFB: Fn(A) -> <F as HKT<B>>::Target,
    {
        match &self {
            Result::Err(e) => F::of(Result::Err(e)),
            Result::Ok(a) => F::of(Result::Ok(traverser(*a))),
        }
    }
}
