use fp_core::applicative::*;
use fp_core::chain::*;
use fp_core::hkt::HKT;
use fp_core::pure::*;

pub trait Monad<A, F, B>: Chain<A, B> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}

impl<A, F, B> Monad<A, F, B> for Option<A> where F: FnOnce(A) -> B {}

#[test]
fn monad_example() {
    let x = Option::of(Some(1)).chain(|x| Some(x + 1));
    assert_eq!(x, Some(2));
}
