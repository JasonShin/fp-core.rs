use fp_core::comonad::*;
use fp_core::extend::*;
use fp_core::extract::*;

#[test]
fn comonad_test() {
    let z = Some(1).extend(|x| x.extract() + 1);
    assert_eq!(z, Some(2));
}
