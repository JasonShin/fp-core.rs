use crate::empty::Empty;
use crate::semigroup::Semigroup;
use std::ops::Add;

pub trait Monoid: Empty + Semigroup {}

impl Monoid for i32 {}
impl Monoid for i64 {}
impl<T: Clone> Monoid for Vec<T> {}
