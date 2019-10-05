use crate::applicative::Applicative;
use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};
use crate::foldable::{ Foldable, fold_map };

pub trait Traverse<A, B>: Foldable<B> {
    fn traverse<AB, F, G>(&self, ap: AB, f: F) -> <Self as HKT<G, <Self as HKT<F, B>>::Target>>::Target
    where
        AB: Applicative<A, B>,
        F: Fn(A) -> <Self as HKT<G, B>>::Target;
}

impl<A, B> Traverse<A, B> for Vec<Option<A>> {
    fn traverse<AB, F, G>(&self, ap: AB, f: F) -> Self::Target where
        AB: Applicative<A, B>,
        F: Fn(A) -> <Self as HKT<G, B>>::Target {
        let result = self.iter().flat_map(|opt| opt.unwrap());
    }
}

#[test]
fn traverse_test() {
    let x = vec![Some(1), Some(2), Some(3)];
}

/*
trait Traverse<A, B, F, G>: HKT<G, B> + HKT3<G, F, B> + Applicative<A, F, B>
    where
        F: FnOnce(A) -> B,
{
    fn traverse<FB>(&self, f: FB) -> <Self as HKT3<G, F, B>>::Error
    where
        FB: FnOnce(A) -> <Self as HKT<G, B>>::Target;
}
*/
