use crate::functor::Functor;
use crate::hkt::HKT;

type Applicator<B, S: HKT<B>> = <S as HKT<Box<Fn(S::Current) -> B>>>::Target;

pub trait Apply<B>: Functor<B> + HKT<Box<Fn(<Self as HKT<B>>::Current) -> B>> {
    fn ap(self, f: Applicator<B, Self>) -> <Self as HKT<B>>::Target;
}

impl<A, B> Apply<B> for Option<A> {
    fn ap(self, f: Applicator<B, Self>) -> <Self as HKT<B>>::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
