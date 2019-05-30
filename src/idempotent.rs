#[test]
fn idempotent() {
    let x: i32 = 12;
    let y = i32::abs(x);
    let z = i32:abs(y);
    assert_eq!(y, z);
}