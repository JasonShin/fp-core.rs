use crate::functor_example::{Functor, HKT};

trait Extend<A, B>: Functor<A, B> + Sized {
    fn extend<W>(self, f: W) -> <Self as HKT<A, B>>::Target
    where
        W: FnOnce(Self) -> B;
}

impl<A, B> Extend<A, B> for Option<A> {
    fn extend<W>(self, f: W) -> Self::Target
    where
        W: FnOnce(Self) -> B,
    {
        self.map(|x| f(Some(x)))
    }
}

trait Extract<A> {
    fn extract(self) -> A;
}

impl<A> Extract<A> for Option<A> {
    fn extract(self) -> A {
        self.unwrap() // is there a better way to achieve this?
    }
}

trait Comonad<A, B>: Extend<A, B> + Extract<A> {}

#[test]
fn comonad_test() {
    let z = Some(1).extend(|x| x.extract() + 1);
    assert_eq!(z, Some(2));
}
