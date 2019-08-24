use fp_core::foldable::*;

/*
// Check out foldable.rs in fp-core
pub trait Foldable<A, B>: HKT<A, B> {
    fn reduce<F>(b: B, ba: F) -> <Self as HKT<A, B>>::Target
    where
        F: FnOnce(B, A) -> (B, B);
}
*/

#[test]
fn foldable_example() {
    let k = vec![1, 2, 3];
    let result = k.reduce(0, |i, acc| i + acc);
    assert_eq!(result, 6);
}

/*
#[test]
fn fold_map_example() {
    let k = vec![Some(1 as i64), Some(2 as i64), Some(3 as i64), None];
    let result = fold_map(k, |&opt| if let Some(x) = opt { x } else { 0 });
    assert_eq!(result, 6);
}
*/
