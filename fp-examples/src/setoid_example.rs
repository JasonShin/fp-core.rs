trait Setoid {
    fn equals(&self, other: &Self) -> bool;
}

impl Setoid for Vec<i32> {
    fn equals(&self, other: &Self) -> bool {
        self.len() == other.len()
    }
}

impl Setoid for &str {
    fn equals(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

#[test]
fn setoid_example() {
    assert_eq!(vec![1, 2].equals(&vec![1, 2]), true);
    assert_eq!(Setoid::equals(&"test", &"test"), true);
}
