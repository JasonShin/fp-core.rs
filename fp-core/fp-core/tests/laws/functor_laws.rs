// Functor Laws Tests
// These tests verify that Functor implementations satisfy the mathematical laws:
// 1. Identity Law: fmap(id) == id
// 2. Composition Law: fmap(g . f) == fmap(f) . fmap(g)

use fp_core::functor::Functor;

// Identity Law Tests
#[test]
fn functor_law_identity_option_some() {
    let fa = Some(42);
    let identity = |x: i32| x;
    assert_eq!(fa.fmap(identity), fa);
}

#[test]
fn functor_law_identity_option_none() {
    let fa: Option<i32> = None;
    let identity = |x: i32| x;
    assert_eq!(fa.fmap(identity), fa);
}

#[test]
fn functor_law_identity_result_ok() {
    let fa: Result<i32, String> = Ok(42);
    let identity = |x: i32| x;
    assert_eq!(fa.fmap(identity), Ok(42));
}

#[test]
fn functor_law_identity_result_err() {
    let fa: Result<i32, String> = Err("error".to_string());
    let identity = |x: i32| x;
    assert_eq!(fa.fmap(identity), Err("error".to_string()));
}

// Composition Law Tests
#[test]
fn functor_law_composition_option() {
    let fa = Some(5);
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    // fmap(g . f) == fmap(f) . fmap(g)
    let composed = fa.fmap(|x| g(f(x)));
    let sequential = Some(5).fmap(f).fmap(g);

    assert_eq!(composed, sequential);
}

#[test]
fn functor_law_composition_result() {
    let fa: Result<i32, String> = Ok(5);
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    let composed = fa.fmap(|x| g(f(x)));
    let sequential: Result<i32, String> = Ok(5).fmap(f).fmap(g);

    assert_eq!(composed, sequential);
}

#[test]
fn functor_law_composition_with_type_change() {
    let fa = Some(10);
    let f = |x: i32| x * 2;
    let g = |x: i32| x.to_string();

    let composed = fa.fmap(|x| g(f(x)));
    let sequential = Some(10).fmap(f).fmap(g);

    assert_eq!(composed, sequential);
}

#[test]
fn functor_law_composition_none_preserves() {
    let fa: Option<i32> = None;
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    let composed = fa.fmap(|x| g(f(x)));
    let sequential: Option<i32> = None.fmap(f).fmap(g);

    assert_eq!(composed, sequential);
}

#[test]
fn functor_law_composition_error_preserves() {
    let fa: Result<i32, String> = Err("error".to_string());
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    let composed = fa.fmap(|x| g(f(x)));
    let sequential: Result<i32, String> = Err("error".to_string()).fmap(f).fmap(g);

    assert_eq!(composed, sequential);
}

// Property: Functors preserve structure
#[test]
fn functor_preserves_structure_option() {
    // Mapping over None should always yield None
    let none: Option<i32> = None;
    assert_eq!(none.fmap(|x| x * 2), None);
    assert_eq!(none.fmap(|x: i32| x.to_string()), None);
}

#[test]
fn functor_preserves_structure_result() {
    // Mapping over Err should preserve the error
    let err: Result<i32, String> = Err("test error".to_string());
    assert_eq!(err.fmap(|x| x * 2), Err("test error".to_string()));
}

// Advanced composition tests
#[test]
fn functor_law_multiple_composition() {
    let fa = Some(2);
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;
    let h = |x: i32| x - 1;

    // fmap(h . g . f)
    let composed = fa.fmap(|x| h(g(f(x))));

    // fmap(f) . fmap(g) . fmap(h)
    let sequential = Some(2).fmap(f).fmap(g).fmap(h);

    assert_eq!(composed, sequential);
}

#[test]
fn functor_law_identity_is_noop() {
    // Verifying that identity truly does nothing
    let values = vec![Some(1), Some(2), None, Some(42)];

    for val in values {
        assert_eq!(val.fmap(|x| x), val);
    }
}
