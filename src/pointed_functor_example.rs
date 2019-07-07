#[derive(Debug, PartialEq, Eq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

impl<T> Maybe<T> {
    fn of(x: T) -> Self {
        Maybe::Just(x)
    }
}

#[test]
fn pointed_functor_example() {
    let pointed_functor = Maybe::of(1);

    assert_eq!(pointed_functor, Maybe::Just(1));
}
