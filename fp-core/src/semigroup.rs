pub trait Semigroup {
    fn combine(&self, b: &Self) -> Self;
}

impl Semigroup for Vec<i32> {
    fn combine(&self, b: &Self) -> Vec<i32> {
        concat(vec![self.clone(), b.clone()])
    }
}
