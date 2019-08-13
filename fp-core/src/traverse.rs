use crate::applicative::Applicative;
use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

trait Traverse<A, B, F, G>: HKT<G, B> + HKT3<G, F, B> + Applicative<A, F, B> {
    fn traverse<FB>(&self, f: FB) -> <Self as HKT3<G, F, B>>::Target2
    where
        FB: FnOnce(A) -> <Self as HKT<G, B>>::Target;
}
