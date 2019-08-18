use fp_core::compose::*;
use fp_core::identity::*;
use fp_core::semigroup::*;

fn foo(a: i32) -> i32 {
    a + 20
}

#[test]
fn monoid_example() {
    let z = 1 + 1;
    let x = 1 + (2 + 3) == (1 + 2) + 3;
    let y = [vec![1, 2, 3], vec![4, 5, 6]].concat();
    let u = [vec![1, 2], vec![]].concat();
    let i = compose!(foo, identity)(1) == compose!(identity, foo)(1);
    assert_eq!(z, 2);
    assert_eq!(x, true);
    assert_eq!(y, vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(u, vec![1, 2]);
    assert_eq!(i, true);

    let j = 1i32.combine(2i32);
    assert_eq!(j, 3i32);
}
