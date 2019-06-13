trait Setoid<'a, A, B>
    where
        A: 'a {
    fn equals(&'a self, b: B) -> bool;
}

impl<'a, A, B> Setoid<'a, A, B> for Vec<A>
    where
        A: 'a {
    fn equals(&'a self, b: Vec<A>) -> bool {
        return (*self.len() == b.len()) &&
            self.iter().zip(b).all(
                |x, y| (x.is_nan() && y.is_nan()) || (x == y)
            );
    }
}

#[test]
fn setoid_example() {

    let result = Vec::equals(&vec![1, 2], vec![1, 2]);
    print!("checking {}", result);
    assert_eq!(1, 2);
}