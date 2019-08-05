#[test]
fn adt_example() {
    #[allow(dead_code)]
    enum WeakLogicValues {
        True(bool),
        False(bool),
        HalfTrue(bool),
    }
    // WeakLogicValues = bool + otherbool + anotherbool

    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
    }
}
