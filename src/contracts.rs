#[test]
fn contracts_example() {
    let contract = |x: &i32| -> bool { x > &10 };

    let add_one = |x: &i32| -> Result<i32, String> {
        if contract(x) {
            Ok(x + 1)
        }
        Err("Cannot add one".to_string())
    };

    let expected = 12;
    match add_one(&11) {
        Ok(x) => assert_eq!(x, expected),
        _ => panic!("Failed!"),
    }
}
