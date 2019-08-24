use crate::functor::Functor;
use crate::hkt::{HKT, HKT2, HKT3, Nothing};
use std::any::Any;

type Applicator<A, B, S: HKT2<A, B>> = <S as HKT<Box<Fn(<S as HKT<A>>::Source) -> B>>>::Source;

// TODO: Need HKT for applicator
pub trait Apply<A, B>: Functor<A, B> + HKT<Box<Fn(<Self as HKT<B>>::Source) -> B>>
{
    fn ap(self, f: Applicator<A, B, Self>) -> Self::Target;
}

impl<A, B> Apply<A, B> for Option<A>
{
    fn ap(self, f: Applicator<A, B, Self>) -> Self::Target
    {
        self.and_then(|v| f.map(|z| z(v)))
    }
}
