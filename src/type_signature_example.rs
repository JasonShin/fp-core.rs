#[test]
fn type_signature_example() {
    // add :: i32 -> i32 -> i32
    fn add(x: i32) -> impl Fn(i32) -> i32 {
        return move |y| x + y;
    }

    // increment :: i32 -> i32
    fn increment(x: i32) -> i32 {
        return x + 1;
    }

    // call :: (a -> b) -> a -> b
    fn call<A, B>(f: &Fn(A) -> B) -> impl Fn(A) -> B + '_ {
        return move |x| f(x);
    }

    // This time with an explicit lifetime
    fn call2<'a, A, B>(f: &'a dyn Fn(A) -> B) -> impl Fn(A) -> B + 'a {
        move |x| f(x)
    }

    // map :: (a -> b) -> [a] -> [b]
    fn map<A, B>(f: &Fn(A) -> B) -> impl Fn(A) -> B + '_ {
        return move |x| f(x);
    }
}
