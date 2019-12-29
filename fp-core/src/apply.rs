use crate::functor::Functor;
use crate::hkt::HKT;

// This is the type for a container of functions we can apply over.
// In haskell, we say `apply::f a -> f (a -> b) -> f b`, however, in rust
// most `f (a -> b)` also need a size for the `a -> b`, so we box it.
// Furthermore, in rust, the HKT trait knows `a`: `<F<A> as HKT<B>>::Current` is `A`.
// Saying this in every place we'd have to was looking unwieldy, so this type does it for us.
// `Applicator<T, Option<U>>` is `Option<Box<dyn Fn(U) -> T>>`.
type Applicator<B, S: HKT<B>> = <S as HKT<Box<dyn Fn(&<S as HKT<B>>::Current) -> B>>>::Target;

pub trait Apply<B>: Functor<B> + HKT<Box<dyn Fn(&<Self as HKT<B>>::Current) -> B>> {
    fn ap(self, f: &Applicator<B, Self>) -> <Self as HKT<B>>::Target;
}

impl<A, B> Apply<B> for Option<A> {
    fn ap(self, f: &Applicator<B, Self>) -> <Self as HKT<B>>::Target {
        self.and_then(|v| f.as_ref().map(|z| z(&v)))
    }
}

impl<A, B, E: Copy> Apply<B> for Result<A, E> {
    fn ap(self, f: &Applicator<B, Self>) -> <Self as HKT<B>>::Target {
        self.and_then(|v| f.as_ref().map(|z| z(&v)).or_else(move |e| Result::Err(*e)))
    }
}

impl<A, B> Apply<B> for Vec<A> {
    fn ap(self, f: &Applicator<B, Self>) -> <Self as HKT<B>>::Target {
        self.into_iter().flat_map(move |v| f.into_iter().map(move |z| z(&v))).collect()
    }
}
