use crate::extend::Extend;
use crate::extract::Extract;

pub trait Comonad<A, B>: Extend<B> + Extract<A> {}
