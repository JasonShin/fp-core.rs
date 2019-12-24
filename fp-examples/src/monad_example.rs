use fp_core::chain::*;
use fp_core::pure::*;

#[test]
fn monad_example() {
    let x = Option::of(1).chain(|x| Some(x + 1));
    assert_eq!(x, Some(2));
}

#[test]
fn monad_example_on_result() {
    let x = Result::<_, ()>::of(1).chain(|x| Ok(x + 1));
    assert_eq!(x, Ok(2));
}
