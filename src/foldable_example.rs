use rats::foldable::Foldable;
use rats::kind::IntoKind;
use rats::kinds::VecKind;

#[test]
fn foldable_example() {
    let k = vec![1, 2, 3].into_kind();
    let result = VecKind::fold_right(k, 0, |(i, acc)| i + acc);
    assert_eq!(result, 6);
}
