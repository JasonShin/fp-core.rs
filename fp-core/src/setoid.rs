pub trait Setoid {
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
