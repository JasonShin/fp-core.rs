fn endomorphism_example() {
    let uppercase = |x: &str| x.to_uppercase();
    let decrement = |x: i32| x - 1;

    assert_eq!(uppercase("abc"), "ABC".to_string());
    assert_eq!(decrement(1), 0);
}
