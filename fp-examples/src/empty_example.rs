use fp_core::empty::*;

#[test]
fn empty_example() {
    let result = i32::empty();
    assert_eq!(result, 0);

    let result2 = Vec::<i32>::empty();
    assert_eq!(result2, vec![]);
}
