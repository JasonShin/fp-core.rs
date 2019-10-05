use crate::applicative::Applicative;
use crate::foldable::{fold_map, Foldable};
use crate::functor::Functor;
use crate::hkt::{HKT, HKT3};

pub trait Traverse<A, B, F, G>: Foldable<B>
where
    F: Fn(A) -> <Self as HKT<B>>::Target
{
    fn traverse<AB>(f: F) -> <Self as HKT<B>>::Target
    where
        AB: Applicative<A, B>;
}

impl<A, B, F, G> Traverse<A, B, F, G> for Vec<Option<A>>
where
    F: Fn(A) -> <Self as HKT<B>>::Target
{
    fn traverse<AB>(f: F) -> <Self as HKT<B>>::Target
    where
        AB: Applicative<A, B>
    {
        unimplemented!()
    }
}

// Option<Vec<B>>

/*
fn traverse<A, B, F, FA, AP>(t: Vec<Option<A>>) -> ()
where
    FA: Foldable<A>,
    AP: Applicative<A, B>,
    F: Fn(A) -> B,
{
    // let z: Vec<A> = t.iter().map(|x| x.unwrap()).collect();
    // println!("checking!! {:?}", z)
    ()
}
*/

/*
impl<A, B> Traverse<A, B> for Vec<Option<A>> {
    fn traverse<AB, F, G>(&self, ap: AB, f: F) -> Self::Target where
        AB: Applicative<A, B>,
        F: Fn(A) -> <Self as HKT<G, B>>::Target {
        let result = self.reduce(Self::empty())
    }
}
*/

#[test]
fn traverse_test() {
    let x = vec![Some(1), Some(2), Some(3)];
    let z = x.iter().fold(vec![], |mut v, r| {
        match r {
            Some(c) => v.push(c),
            None => panic!("lol")
        }
        v
    });
    println!("zz {:?}", z);
    assert_eq!(1, 2)
}

/*
trait Traverse<A, B, F, G>: HKT<G, B> + HKT3<G, F, B> + Applicative<A, F, B>
    where
        F: FnOnce(A) -> B,
{
    fn traverse<FB>(&self, f: FB) -> <Self as HKT3<G, F, B>>::Error
    where
        FB: FnOnce(A) -> <Self as HKT<G, B>>::Target;
}
*/
