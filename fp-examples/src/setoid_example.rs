#[cfg(test)]
mod example {
    use fp_core::setoid::*;

    #[test]
    fn setoid_example() {
        assert!(vec![1, 2].equals(&vec![1, 2]));
        assert!(Setoid::equals(&"test", &"test"));
    }
}
