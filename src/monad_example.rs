pub trait Functor<'a, A, B, F>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output;

    fn pure(&'a self) -> Self::Output;
    fn
    fn flat_map(&'a self, f: F) -> Self::Output;

}
