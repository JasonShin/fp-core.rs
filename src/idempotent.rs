use std::num;

#[test]
fn idempotent_sort() {
    let sort = |x: Vec<i32>| -> Vec<i32> {
        let mut cloned_x = x.clone();
        cloned_x.sort();
        cloned_x
    };

    let x = vec![2, 1];
    let sorted_x = sort(sort(x.clone()));
    let expected = vec![1, 2];
    assert_eq!(sorted_x, expected);
}

#[test]
fn idempotent_abs() {
    let abs = |x: i32| -> i32 { x.abs() };

    let x: i32 = 10;
    let result = abs(abs(x));
    assert_eq!(result, x);
}
