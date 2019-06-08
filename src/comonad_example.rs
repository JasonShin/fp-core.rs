/*
trait Comonad<A, B>: Extend<A, B>
{
    fn extract(self) -> A;

    // extend should already be provided from `Extend`
    fn extend(self, f: F) -> HKT1<B>
    where
        F: Fn(Self) -> B;
}
*/


#[test]
fn comonad_test() {
    // Check the readme to understand comonad
    // Skipping implementation detail as it require HKT and various other components
    assert!(true);
}
