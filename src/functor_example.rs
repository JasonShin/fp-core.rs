pub trait HKT<A, B> {
    type URI;
    type Target;
}

pub trait Functor2<A, B>: HKT<A, B> {
    fn fmap(self, f: fn(A) -> B) -> <Self as HKT<A, B>>::Target;
}

impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}

impl<A, B> Functor2<A, B> for Option<A> {
    fn fmap(self, f: fn(A) -> B) -> Self::Target {
        self.map(f)
    }
}

#[test]
fn test_functor2() {
    let  z = Option::fmap(Some(1), |x| x + 1).fmap(|x| x + 1);
    assert_eq!(z.unwrap(), 3);
}

/*
// Below is an old implementation
#[derive(Debug, PartialEq, Eq)]
pub enum Maybe<T> {
    Nothing,
    Just(T),
}

#[test]
fn functor_example_1() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(| x | x + 1).collect();

    assert_eq!(v, vec![2, 3, 4]);
}

pub trait Functor<'a, A, B, F>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output;
    fn fmap(&'a self, f: F) -> Self::Output;
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Maybe<A>
    where
        A: 'a,
        F: Fn(&'a A) -> B {

    type Output = Maybe<B>;
    fn fmap(&'a self, f: F) -> Maybe<B> {
        match *self {
            Maybe::Just(ref x) => Maybe::Just(f(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[test]
fn functor_example_2() {
    let just = Maybe::Just(7);
    let nothing = Maybe::fmap(&Maybe::Nothing, |x| x + 1);
    let other = Maybe::fmap(&just, |x| x + 1);
    assert_eq!(nothing, Maybe::Nothing);
    assert_eq!(other, Maybe::Just(8));
}
*/