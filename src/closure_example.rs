#[test]
fn closure() {
    let add_to = | x: i32 | { move | y: i32 | { x + y } };

    let add_to_five = add_to(5);

    assert_eq!(add_to_five(3), 8);
}
