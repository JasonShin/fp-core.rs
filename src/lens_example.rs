trait Lens<S, A> {
    fn get(s: S) -> A;
    fn set(a: A, s: S) -> S;
}

#[test]
fn lens_example() {
    assert_eq!(1, 1);
}
