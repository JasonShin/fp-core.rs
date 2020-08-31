#[test]
fn type_signature_example() {
    // add :: i32 -> i32 -> i32
    #[allow(dead_code)]
    fn add(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    // increment :: i32 -> i32
    #[allow(dead_code)]
    fn increment(x: i32) -> i32 {
        x + 1
    }

    // call :: (a -> b) -> a -> b
    #[allow(dead_code)]
    fn call<A, B>(f: &(dyn Fn(A) -> B)) -> impl Fn(A) -> B + '_ {
        move |x| f(x)
    }

    // This time with an explicit lifetime
    #[allow(dead_code)]
    fn call2<'a, A, B>(f: &'a dyn Fn(A) -> B) -> impl Fn(A) -> B + 'a {
        move |x| f(x)
    }

    // map :: (a -> b) -> [a] -> [b]
    #[allow(dead_code)]
    fn map<A, B>(f: &(dyn Fn(A) -> B)) -> impl Fn(A) -> B + '_ {
        move |x| f(x)
    }
}
