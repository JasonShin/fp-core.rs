#[test]
fn partial_application() {
    fn foo(a: i32, b: i32, c: i32, d: i32, mul: i32, off: i32) -> i32 {
        (a + b*b + c.pow(3) + d.pow(4)) * mul - off
    }

    let bar = partial!(foo(_, _, 10, 42, 10, 10));

    assert_eq!(
        foo(15, 15, 10, 42, 10, 10),
        bar(15, 15)
    );
}
