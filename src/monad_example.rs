use crate::functor_example::HKT;
use crate::applicative_example::{Applicative, Apply};

/*
pub trait Chain<A, F, B>: Applicative<A, F, B>
    where F: FnOnce(A) -> <Self as HKT<A, B>>::Target {
    fn chain(self, f: F) -> <Self as HKT<A, B>>::Target;
}
*/

/*
impl<A, F, B> Chain<A, F, B> for Option<A>
    where F: FnOnce(A) -> <Self as HKT<A, B>>::Target {
    fn chain(self, f: F) -> <Self as HKT<A, B>>::Target {
        self.and_then(f)
    }
}
*/


// NOTE: Should we do this instead?
pub trait Chain<A, B>: HKT<A, B> {
    fn chain<F>(self, f: F) -> <Self as HKT<A, B>>::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> Chain<A, B> for Option<A> {
    fn chain<F>(self, f: F) -> Self::Target
        where F: FnOnce(A) -> <Self as HKT<A, B>>::Target {
        self.and_then(f)
    }
}

pub trait Monad<A, F, B>: Chain<A, B> + Applicative<A, F, B>
    where F: FnOnce(A) -> B {}

impl<A, F, B> Monad<A, F, B> for Option<A>
    where F: FnOnce(A) -> B {}

#[test]
fn monad_example() {
    // check out the Readme file.
    // Monad is going to implemented in PureRust along with HKT, Applicative and Functor implementations
    // let x = Option::of(1).chain(|x| x + 1);
    let x = Monad::of(Some(1)).ap(Some(|x| x + 1));
    assert_eq!(1, 1);
}
