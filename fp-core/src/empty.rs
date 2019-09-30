pub trait Empty {
    fn empty() -> Self;
}

impl Empty for i32 {
    fn empty() -> Self {
        0.into()
    }
}

impl Empty for i64 {
    fn empty() -> Self {
        0.into()
    }
}

impl<T> Empty for Vec<T> {
    fn empty() -> Self {
        Vec::empty()
    }
}
