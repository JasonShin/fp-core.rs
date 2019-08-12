use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

pub trait Apply<A, B>: Functor<A, B>
{
    type FAB;
    type Output;
    fn ap(self, f: Self::FAB) -> Self::Output;
}

impl<A, B> Apply<A, B> for Option<A>
{
    type FAB = Option<dyn Fn(A) -> B>;
    type Output = Option<B>;
    fn ap(self, f: Self::FAB) -> Self::Output {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
