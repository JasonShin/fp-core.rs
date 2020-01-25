use crate::empty::Empty;
use crate::semigroup::Semigroup;

use std::fmt::Debug;

pub trait Monoid: Empty + Semigroup {}

impl Monoid for i32 {}
impl Monoid for i64 {}
impl<T: Clone> Monoid for Vec<T> {}
impl Monoid for String {}
impl<A: Semigroup> Monoid for Option<A> {}
impl<A: Semigroup, E: Empty + Debug> Monoid for Result<A, E> {}
