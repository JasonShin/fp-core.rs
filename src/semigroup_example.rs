trait Semigroup {
    fn combine(&self, b: &Self) -> Self;
}

impl Semigroup for Vec<i32> {
    fn combine(&self, b: &Self) -> Vec<i32> {
        return &self.concat(b);
    }
}

#[test]
fn semigroup_test() {

    println!("semi {:?}", vec![1, 2].combine(&vec![3, 4]));
    assert_eq!(1, 2);
}

