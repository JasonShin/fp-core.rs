use crate::apply::Apply;
use crate::pure::Pure;

pub trait Applicative<B>: Apply<B> + Pure<B> {}

impl<A, B> Applicative<B> for Option<A> {}
impl<A, B> Applicative<B> for Vec<A> {}

// Note on trait bound: look in apply.rs with the
// impl Apply for Result
impl<A, B, E: Copy> Applicative<B> for Result<A, E> {}
