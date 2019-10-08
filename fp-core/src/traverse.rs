use crate::applicative::Applicative;
use crate::foldable::Foldable;
use crate::functor::Functor;
use crate::hkt::{HktInHkt, HKT};

// Re-statement of the Haskell definition (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
pub trait Traversable<B>: Functor<B> + Foldable<B> {
    // A is the source type, F the functor, and AFB a natural transformation
    // from A into F B. Pursuant to Cats in Scala (https://www.scala-exercises.org/cats/traverse),
    // A would be a User, F a future, and Self would be a list.
    fn traverse<A, F, AFB>(
        traverser: AFB,
        traversed: <Self as HKT<B>>::Target,
    ) -> HktInHkt<F, Self, B>
    where
        AFB: Box<Fn(A) -> <F as HKT<B>>::Target>;
}
