trait Setoid<A> {
    fn equals(&self, other: &Self) -> bool;
}

impl<A> Setoid<A> for Vec<A> {
    fn equals(&self, other: &Self) -> bool {
        return self.len() == other.len();
    }
}

#[test]
fn setoid_example() {
    assert_eq!(vec![1, 2].equals(&vec![1, 2]), true);
}
