use fp_core::apply::*;
use fp_core::pure::*;

#[test]
fn applicative_example() {
    let x = Option::of(Some(1)).ap(Some(Box::new(|x| x + 1)));
    assert_eq!(x, Some(2));
}
