use fp_core::traverse::*;

fn div_tuple(tpl: (f32, f32)) -> Option<f32> {
    match tpl {
        (_, 0) => Option::None,
        (x, y) => Option::Some(x / y),
    }
}

#[test]
fn traverse_example() {
    let will_be_some_quotients = vec![(2, 1), (4, 2), (6, 3)];
    assert_eq!(
        will_be_some_quotients.traverse(div_tuple),
        Option::Some(vec![2, 2, 2])
    );

    let will_not_be_some_quotients = vec![(2, 1), (4, 2), (6, 3), (45, 0)];
    assert_eq!(will_not_be_some_quotients.traverse(div_tuple), Option::None);
}
