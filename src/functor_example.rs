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


impl<'a, A, B, C> Functor<'a, A, B, C> for Maybe<A>
    where
        A: 'a,
        F: Fn<&'a, A> -> B {

    type Output = Maybe<B>;
    fn fmap(&'a, self, f: F) {

    }

}



#[test]
fn functor_example_2() {

}
