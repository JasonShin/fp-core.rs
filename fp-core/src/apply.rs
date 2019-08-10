use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

pub trait Apply<F, B>: Functor<B> + HKT3<<Self as HKT<B>>::Current, F, B>
where
    F: FnOnce(<Self as HKT<B>>::Current) -> B,
{
    fn ap(
        self,
        f: <Self as HKT3<<Self as HKT<B>>::Current, F, B>>::Target2,
    ) -> <Self as HKT<B>>::Target;
}

impl<A, F, B> Apply<F, B> for Option<A>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
