#[test]
fn adt_example() {
    enum WeakLogicValues {
        True(bool),
        False(bool),
        HalfTrue(bool),
    }
    // WeakLogicValues = bool + otherbool + anotherbool

    struct Point {
        x: i32,
        y: i32,
    }
}
