pub struct Nothing {}

// *
// For example Option<A>
pub trait HKT<A> {
    type Source;
}

impl<A> HKT<A> for Option<A> {
    type Source = Self;
}

// * -> *
// For example Option<A> -> Option<B>
pub trait HKT2<A, B>: HKT<A> {
    type Source;
    type Target;
}

impl<A, B> HKT2<A, B> for Option<A> {
    type Source = Self;
    type Target = Option<B>;
}

// * -> * -> *
// For example Result<A, Error> -> Result<B, Error>
pub trait HKT3<A, B, C>: HKT2<A, B> {
    type Error;
}

impl<A, B, C> HKT3<A, B, C> for Option<A> {
    type Error = Option<B>;
}
