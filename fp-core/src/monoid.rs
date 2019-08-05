use crate::applicative::Applicative;
use crate::empty::Empty;

trait Monoid<A, F, B>: Empty<A> + Applicative<A, F, B>
where
    F: FnOnce(A) -> B,
{
}
