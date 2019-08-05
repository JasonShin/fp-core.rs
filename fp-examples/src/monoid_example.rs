use fp_core::applicative::Applicative;

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

fn identity<A>(a: A) -> A {
    a
}

fn foo(a: i32) -> i32 {
    a + 20
}

trait Empty<A> {
    fn empty() -> A;
}

trait Monoid<A, F, B>: Empty<A> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}

#[test]
fn monoid_example() {
    let z = 1 + 1;
    let x = 1 + (2 + 3) == (1 + 2) + 3;
    let y = [vec![1, 2, 3], vec![4, 5, 6]].concat();
    let u = [vec![1, 2], vec![]].concat();
    let i = compose!(foo, identity)(1) == compose!(identity, foo)(1);
    assert_eq!(z, 2);
    assert_eq!(x, true);
    assert_eq!(y, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(u, vec![1, 2]);
    assert_eq!(i, true);
}
