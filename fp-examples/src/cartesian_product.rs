use crate::functor_example::{Functor};

#![test]
fn cartesian_product<A, B>(s: impl Iterator<Item=A>,
                           t: impl Iterator<Item=B>)
    -> impl Iterator<Item=(A, B)>{
    s.flat_map(|x| t.map(|y| (x, y)))
}

pub trait Diagram<A>{
    fn objects(&self) -> Vec<A>;
    fn arrows(&self) -> Vec<(A, A)>;

    type Embedding<C> = Functor<Self, C, URI=Self, Target=C>;
}

pub trait Cone<A>{
    fn underlying_diagram(&self) -> impl Diagram<A>;
    fn object_at_tip(&self) -> A;
    fn arrows_into_diagram(&self) -> Vec<(A, A)>;
}

pub trait Limit<A>: Cone<A>{
    fn unique_arrow_to_cone(&self, other_cone: impl Cone<A>)
        -> Vec<(A, A)>;
}
