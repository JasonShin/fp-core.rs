#[test]
fn contracts_example() {
    let contract = | x: &i32 | -> bool {
        return x > &10;
    };

    let add_one = | x: &i32 | -> Result<i32, String> {
        if contract(x) {
            return Ok(x + 1);
        }
        return Err("Cannot add one".to_string());
    };

    let expected = 12;
    match add_one(&11) {
        Ok(x) => assert_eq!(x, expected),
        _ => panic!("Failed!")
    }
}
