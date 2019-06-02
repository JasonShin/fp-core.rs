#[test]
fn lambdas() {
    fn  increment(i: i32) -> i32 { i + 1 }
    let closure_annotated = |i: i32| { i + 1 };
    let closure_inferred = |i| i + 1;
    
    let inc = increment(3);
    let ca = closure_annotated(3);
    let ci = closure_inferred(3);

    assert_eq!(inc, 4);
    assert_eq!(ca, 4);
    assert_eq!(ci, 4);
}
