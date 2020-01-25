use crate::applicative::{Applicative, Applicator};
use crate::foldable::Foldable;
use crate::functor::Functor;
use crate::hkt::{HktInHkt, HKT};
use crate::pure::Pure;

use std::fmt::Debug;

// Re-statement of the Haskell definition
// (http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html)
// that uses sequence as the basis for traverse, that is:
//     class (Functor t, Foldable t) => Traverse t where
//       sequence::(Applicative f) => t (f b) -> f (t b)
//
// Unfortunately, Rust doesn't handle a "for all" in the same way, so to Rust,
// we must know that T is functorial over F<B> and just B and F is Applicative
// for B and T<B>.
pub trait Traversable<B, F>:
    Functor<B> + Foldable<B> + Functor<<F as HKT<B>>::Target> + Foldable<<F as HKT<B>>::Target>
where
    F: Applicative<B> + Applicative<<Self as HKT<B>>::Target>,
    Self:
        Functor<B> + Foldable<B> + Functor<<F as HKT<B>>::Target> + Foldable<<F as HKT<B>>::Target>,
{
    fn sequence(tfb: HktInHkt<Self, F, B>) -> HktInHkt<F, Self, B>;
}

impl<A, B, F> Traversable<B, F> for Option<A>
where
    F: Applicative<B> + Applicative<Option<B>> + Pure<Applicator<B, Option<A>>>,
{
    fn sequence(tbf: HktInHkt<Self, F, B>) -> HktInHkt<F, Self, B> {
        match tbf {
            Option::None => F::of(Option::None),
            Option::Some(fb) => fb.ap(F::of(Box::new(|v| Option::Some(v)))),
        }
    }
}

impl<A, B, F> Traversable<B, F> for Vec<A>
where
    F: Applicative<B> + Applicative<Vec<B>> + Pure<Applicator<B, Vec<A>>>,
{
    fn traverse<AFB>(&self, traverser: Box<AFB>) -> HktInHkt<F, Self, B>
    where
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

    fn sequence(tbf: HktInHkt<Self, F, B>) -> HktInHkt<F, Self, B> {
        let acc = F::of(vec![]);
        for item in tbf {
            acc = item.ap(F::of(Box::new(|b| acc.ap(F::of(Box::new(|v| v.extend(&[b])))))))
        }
        return acc;
    }
}

impl<A, B, E: Debug, F> Traversable<B, F> for Result<A, E>
where
    F: Applicative<B> + Applicative<Result<B, E>> + Pure<Applicator<B, Result<A, E>>>,
{
    fn sequence(tbf: HktInHkt<Self, F, B>) -> HktInHkt<F, Self, B> {
        match tbf {
            Result::Err(e) => F::of(Result::Err(e)),
            Result::Ok(fb) => fb.ap(F::of(Box::new(|v| Result::Ok(v)))),
        }
    }
}
