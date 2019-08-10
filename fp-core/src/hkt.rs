pub trait HKT<A, B> {
    type URI;
    type Target;
}

impl<A, B> HKT<A, B> for Option<A> {
    type URI = Self;
    type Target = Option<B>;
}

pub trait HKT3<A, B, C> {
    type Target2;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Target2 = Option<B>;
}
