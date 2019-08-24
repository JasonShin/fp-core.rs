use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

pub trait Apply<A, F, B>: Functor<A, B> + HKT3<A, F, B>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Error) -> <Self as HKT<A, B>>::Target;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
where
    F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Error) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
