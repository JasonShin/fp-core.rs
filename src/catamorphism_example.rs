#[test]
fn catamorphism_example() {
    let sum = |xs: Vec<i32>| xs.iter().fold(0, |mut sum, &val| { sum += val; sum });

    assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
}