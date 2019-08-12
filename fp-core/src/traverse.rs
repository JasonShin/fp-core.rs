use crate::functor::Functor;
use crate::applicative::Applicative;
use crate::hkt::{HKT3, HKT};

trait Traverse<A, B, F, G>: Functor<A, B> + Applicative<A, F, B> {
    fn traverse<FA>(&self) -> <Self as HKT3<G, F, B>>::Target2
    where
        FA: FnOnce(A) -> <Self as HKT<G, B>>::Target;
}
