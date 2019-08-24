// * -> *
// For example Option<A> -> Option<B>
pub trait HKT<B> {
    type Source;
    type Target;
}

impl<A, B> HKT<B> for Option<A> {
    type Source = Self;
    type Target = Option<B>;
}

// * -> * -> *
// For example Result<A, Error> -> Result<B, Error>
pub trait HKT3<B, C>: HKT<B> {
    type Error;
}

impl<A, B, C> HKT3<B, C> for Option<A> {
    type Error = Option<B>;
}
