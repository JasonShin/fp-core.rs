#[cfg(test)]
mod example {
    use fp_core::apply::*;
    use fp_core::pure::*;

    #[test]
    fn applicative_example() {
        let applicator = Some(Box::new(move |x: &i32| x + 1));
        let x = Option::of(1).ap(&applicator);
        assert_eq!(x, Some(2));
    }

    #[test]
    fn applicative_example_on_result() {
        let applicator = Ok(Box::new(move |x: &i32| x + 1));
        let x = Result::<_, ()>::of(1).ap(&applicator);
        assert_eq!(x, Ok(2));
    }
}
