use fp_core::compose::*;

#[test]
fn function_composition() {
    let add = |x: i32| x + 2;
    let multiply = |x: i32| x * 2;
    let divide = |x: i32| x / 2;

    let intermediate = compose!(add, multiply, divide);

    let subtract = |x: i32| x - 1;

    let finally = compose!(intermediate, subtract);

    let expected = 11;
    let result = finally(10);

    assert_eq!(result, expected);
}
