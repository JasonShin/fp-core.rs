#[cfg(test)]
mod example {
    use fp_core::empty::*;

    #[test]
    fn empty_example() {
        let empty_vec = Vec::<i32>::empty();
        assert_eq!(empty_vec, vec![])
    }
}
