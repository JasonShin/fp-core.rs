#[cfg(test)]
mod example {
    use fp_core::empty::*;

    #[test]
    fn empty_example_vec() {
        let empty_vec = Vec::<i32>::empty();
        assert_eq!(empty_vec, vec![])
    }

    #[test]
    fn empty_example_string() {
        let empty_str = String::empty();
        assert_eq!(empty_str, "".to_string())
    }
}
