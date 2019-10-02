use fp_core::apply::*;
use fp_core::pure::*;

#[test]
fn applicative_example() {
    let x = Option::of(1).ap(Some(Box::new(|x| x + 1)));
    assert_eq!(x, Some(2));
}

#[test]
fn applicative_example_on_result() {
    let x = Result::<_,()>::of(1).ap(Ok(Box::new(|x| x + 1)));
    assert_eq!(x, Ok(2));
}
