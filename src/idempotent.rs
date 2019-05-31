#[test]
fn idempotent() {
    let sort = | x: Vec<i32> | -> Vec<i32> {
        let mut cloned_x = x.clone();
        cloned_x.sort();
        return cloned_x;
    };

    let x = vec![2 ,1];
    let sorted_x = sort(sort(x.clone()));
    let expected = vec![1, 2];
    assert_eq!(sorted_x, expected);
}