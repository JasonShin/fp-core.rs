/*
// Note that below are just example code. We no longer use manual semigroup implementation.
// Instead we favour Add, Mul, Sub and etc from std.
use fp_core::semigroup::*;

#[test]
fn semigroup_test() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let c = vec![5, 6];
    assert_eq!(vec![1, 2].combine(&vec![3, 4]), vec![1, 2, 3, 4],);
    assert_eq!(a.combine(&b).combine(&c), a.combine(&b.combine(&c)),);
}
*/
