pub trait Extract<A> {
    fn extract(self) -> A;
}

impl<A> Extract<A> for Option<A> {
    fn extract(self) -> A {
        self.unwrap() // is there a better way to achieve this?
    }
}
