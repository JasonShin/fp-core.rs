#[test]
fn currying() {
    fn add(x: i32) -> impl Fn(i32)-> i32 {
        return move |y| x + y;
    }

    let add5 = add(5);
    let result = add5(10);
    assert_eq!(result, 15);
}
