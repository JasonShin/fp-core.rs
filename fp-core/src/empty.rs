pub trait Empty {
    fn empty() -> Self;
}

impl<I> Empty for I
where
    I: From<i32>,
{
    fn empty() -> I {
        0.into()
    }
}
