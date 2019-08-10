pub trait Lens<S, A> {
    fn over(s: &S, f: &Fn(Option<&A>) -> A) -> S {
        let result: A = f(Self::get(s));
        Self::set(result, &s)
    }
    fn get(s: &S) -> Option<&A>;
    fn set(a: A, s: &S) -> S;
}
