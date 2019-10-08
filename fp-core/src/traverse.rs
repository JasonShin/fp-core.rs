use crate::applicative::Applicative;
use crate::foldable::Foldable;
use crate::functor::Functor;
use crate::hkt::{HktInHkt, HKT};

// Re-statement of the Haskell definition (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
// TODO: who actually is "self"? They should be different to maintain that we're emitting an HKT,
// so self is arguably Self<A> for all A. Or is self really just "traversed"?
pub trait Traversable<B>: Functor<B> + Foldable<B> {
    // A is the source type, F the functor, and AFB a natural transformation
    // from A into F<B>. Pursuant to Cats in Scala (https://www.scala-exercises.org/cats/traverse),
    // A would be a User, F a future, and Self would be a list. AFB would be the sort of "user
    // promise" function in the example. Self, perhaps, is just a witness for now.
    fn traverse<A, F, AFB>(
        self,
        traverser: AFB,
        traversed: <Self as HKT<B>>::Target,
    ) -> HktInHkt<F, Self, B>
    where
        F: Applicative<B>,
        AFB: Box<Fn(A) -> <F as HKT<B>>::Target>;
}

pub fn sequenceA<T, F, A>(wrong_order: HktInHkt<T, F, A>) -> HktInHkt<F, T, A>
where
    T: Traversable<A>,
    F: Applicative<A>,
{
    wrong_order.traverse(|x| x, wrong_order)
}
