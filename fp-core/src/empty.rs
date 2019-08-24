pub trait Empty {
    fn empty() -> Self;
}

/*
impl<I> Empty for I
where
    I: From<i32>,
{
    fn empty() -> I {
        0.into()
    }
}*/

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
