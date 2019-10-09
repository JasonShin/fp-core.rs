use crate::applicative::Applicative;
use crate::foldable::Foldable;
use crate::functor::Functor;
use crate::hkt::{HktInHkt, HKT};

// Re-statement of the Haskell definition
// (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
pub trait Traversable<B>: Functor<B> + Foldable<B> {
    // A is the source type, F the functor, and AFB a natural transformation
    // from A into F<B>. Pursuant to Cats in Scala (https://www.scala-exercises.org/cats/traverse),
    // A would be a User, F a future, and Self would be a list. AFB would be the sort of "user
    // promise" function in the example.
    fn traverse<A, F, AFB>(&self, traverser: AFB) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B>,
        AFB: Box<Fn(A) -> <F as HKT<B>>::Target>;
}

// Re-statement of the Haskell function
// (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
pub fn sequenceA<T, F, A>(wrong_order: HktInHkt<T, F, A>) -> HktInHkt<F, T, A>
where
    T: Traversable<A>,
    F: Applicative<A>,
{
    wrong_order.traverse(|x| x)
}

impl<A, B> Traversable<B> for Option<A> {
    fn traverse<A, F, AFB>(&self, traverser: AFB) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B>,
        AFB: Box<Fn(A) -> <F as HKT<B>>::Target>,
    {
        match &self {
            Option::None => Option::None,
            Option::Some(a) => Option::Some(traverser(a)),
        }
    }
}

impl<A, B, E> Traversable<B> for Result<A, E> {
    fn traverse<A, F, AFB>(&self, traverser: AFB) -> HktInHkt<F, Self, B>
    // (Self is a HKT<B> so this works out)
    where
        F: Applicative<B>,
        AFB: Box<Fn(A) -> <F as HKT<B>>::Target>,
    {
        match &self {
            Result::Err(e) => Result::Err(e),
            Result::Ok(a) => Result::Ok(traverser(a)),
        }
    }
}

