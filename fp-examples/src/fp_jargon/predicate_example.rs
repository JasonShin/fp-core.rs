#[test]
fn predicate_example() {
    let predicate = |a: &i32| a.clone() > 2;

    let result = (vec![1, 2, 3, 4])
        .into_iter()
        .filter(predicate)
        .collect::<Vec<i32>>();

    assert_eq!(result, vec![3, 4]);
}
