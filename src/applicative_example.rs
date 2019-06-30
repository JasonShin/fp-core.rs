use crate::functor_example::{Functor, HKT};

trait HKT3<A, B, C> {
    type Target2;
}

trait Apply<A, F, B> : Functor<A, B> + HKT3<A, F, B>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: <Self as HKT3<A, F, B>>::Target2) -> <Self as HKT<A, B>>::Target;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}

impl<A, F, B> Apply<A, F, B> for Option<A>
    where F: FnOnce(A) -> B,
{
    fn ap(self, f: Self::Target2) -> Self::Target {
        self.and_then(|v| f.map(|z| z(v)))
    }
}

trait Applicative<A, F, B> : Apply<A, F, B>
    where F: FnOnce(A) -> B,
{
    fn of(a: A) -> <Self as HKT<A, B>>::Target;
}

