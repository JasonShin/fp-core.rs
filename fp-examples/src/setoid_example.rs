use fp_core::setoid::*;

#[test]
fn setoid_example() {
    assert_eq!(vec![1, 2].equals(&vec![1, 2]), true);
    assert_eq!(Setoid::equals(&"test", &"test"), true);
}
