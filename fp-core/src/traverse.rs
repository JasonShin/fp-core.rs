use crate::applicative::Applicative;
use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

trait Traverse<A, B, F, G>: HKT<G, B> + HKT3<G, F, B> + Applicative<A, F, B>
    where
        F: FnOnce(A) -> B,
{
    fn traverse<FB>(&self, f: FB) -> <Self as HKT3<G, F, B>>::Error
    where
        FB: FnOnce(A) -> <Self as HKT<G, B>>::Target;
}
