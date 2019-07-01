use itertools::concat;

trait Semigroup {
    fn combine(&self, b: &Self) -> Self;
}

impl Semigroup for Vec<i32> {
    fn combine(&self, b: &Self) -> Vec<i32> {
        return concat(vec![self.clone(), b.clone()]);
    }
}

#[test]
fn semigroup_test() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let c = vec![5, 6];
    assert_eq!(vec![1, 2].combine(&vec![3, 4]), vec![1, 2, 3, 4],);
    assert_eq!(a.combine(&b).combine(&c), a.combine(&b.combine(&c)),);
}
