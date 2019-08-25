pub trait Empty {
    fn empty() -> Self;
}

impl Empty for i32 {
    fn empty() -> Self {
        0
    }
}

impl<A> Empty for Vec<A> {
    fn empty() -> Self {
        vec![]
    }
}
