pub trait HKT<A, B> {
    type URI;
    type Target;
}

impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}
