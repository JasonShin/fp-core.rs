use fp_core::extend::*;
use fp_core::extract::*;
use fp_core::functor::Functor;
use fp_core::hkt::HKT;

trait Comonad<A, B>: Extend<A, B> + Extract<A> {}

#[test]
fn comonad_test() {
    let z = Some(1).extend(|x| x.extract() + 1);
    assert_eq!(z, Some(2));
}
