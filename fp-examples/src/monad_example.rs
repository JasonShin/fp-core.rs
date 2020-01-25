#[cfg(test)]
mod example {
    use fp_core::chain::*;
    use fp_core::pure::*;

    #[test]
    fn monad_example() {
        let x = Option::<i32>::of(1).chain(|x| Some(x + 1));
        assert_eq!(x, Some(2));
    }

    #[test]
    fn monad_example_on_result() {
        let x = Result::<i32, ()>::of(1).chain(|x| Ok(x + 1));
        assert_eq!(x, Ok(2));
    }
}
