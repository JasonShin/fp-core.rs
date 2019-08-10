use rats::foldable::Foldable;
use rats::kind::IntoKind;
use rats::kinds::VecKind;

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
    let k = vec![1, 2, 3].into_kind();
    let result = VecKind::fold_right(k, 0, |(i, acc)| i + acc);
    assert_eq!(result, 6);
}
