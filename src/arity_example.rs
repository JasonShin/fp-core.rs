#[test]
fn arity() {
    let sum = |a: i32, b: i32| a + b;
    let result = sum(1, 2);
    assert_eq!(result, 3);
}
