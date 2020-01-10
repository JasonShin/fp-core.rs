#[cfg(test)]
mod example {
    use fp_core::apply::*;
    use fp_core::pure::*;

    #[test]
    fn applicative_example() {
        let applicator: Applicator<i32, Option<i32>> = Some(Box::new(move |x: &i32| x + 1));
        let x = Option::<i32>::of(1).ap(&applicator);
        assert_eq!(x, Some(2));
    }

    #[test]
    fn applicative_example_on_result() {
        let applicator: Applicator<i32, Result<i32, ()>> = Ok(Box::new(move |x: &i32| x + 1));
        let x = Result::<i32, ()>::of(1).ap(&applicator);
        assert_eq!(x, Ok(2));
    }
}
