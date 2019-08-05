macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[test]
fn function_composition() {
    let add = |x: i32| x + 2;
    let multiply = |x: i32| x * 2;
    let divide = |x: i32| x / 2;

    let intermediate = compose!(add, multiply, divide);

    let subtract = |x: i32| x - 1;

    let finally = compose!(intermediate, subtract);

    let expected = 11;
    let result = finally(10);

    assert_eq!(result, expected);
}
