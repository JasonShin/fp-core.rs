use fp_core::functor::Functor;

#[test]
fn test_option_functor_some() {
    let x = Some(5);
    let result = x.fmap(|n| n * 2);
    assert_eq!(result, Some(10));
}

#[test]
fn test_option_functor_none() {
    let x: Option<i32> = None;
    let result = x.fmap(|n| n * 2);
    assert_eq!(result, None);
}

#[test]
fn test_option_functor_chain() {
    let x = Some(5);
    let result = x.fmap(|n| n * 2).fmap(|n| n + 1);
    assert_eq!(result, Some(11));
}

#[test]
fn test_option_functor_type_change() {
    let x = Some(5);
    let result = x.fmap(|n| n.to_string());
    assert_eq!(result, Some("5".to_string()));
}

#[test]
fn test_result_functor_ok() {
    let x: Result<i32, String> = Ok(10);
    let result = x.fmap(|n| n * 3);
    assert_eq!(result, Ok(30));
}

#[test]
fn test_result_functor_err() {
    let x: Result<i32, String> = Err("error".to_string());
    let result = x.fmap(|n| n * 3);
    assert_eq!(result, Err("error".to_string()));
}

#[test]
fn test_result_functor_chain() {
    let x: Result<i32, String> = Ok(5);
    let result = x.fmap(|n| n * 2).fmap(|n| n + 10);
    assert_eq!(result, Ok(20));
}

#[test]
fn test_result_functor_preserves_error() {
    let x: Result<i32, String> = Err("initial error".to_string());
    let result = x.fmap(|n| n * 2).fmap(|n| n + 10);
    assert_eq!(result, Err("initial error".to_string()));
}

#[test]
fn test_result_functor_type_change() {
    let x: Result<i32, String> = Ok(42);
    let result = x.fmap(|n| format!("Number: {}", n));
    assert_eq!(result, Ok("Number: 42".to_string()));
}

#[test]
fn test_functor_with_closure() {
    let multiplier = 3;
    let x = Some(5);
    let result = x.fmap(|n| n * multiplier);
    assert_eq!(result, Some(15));
}

#[test]
fn test_functor_identity_law_option() {
    // Identity law: fmap(id) == id
    let x = Some(42);
    let result = x.fmap(|n| n);
    assert_eq!(result, Some(42));
}

#[test]
fn test_functor_identity_law_result() {
    let x: Result<i32, String> = Ok(42);
    let result = x.fmap(|n| n);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_functor_composition_law_option() {
    // Composition law: fmap(g . f) == fmap(f).fmap(g)
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    let x1 = Some(5);
    let x2 = Some(5);

    let composed = x1.fmap(|x| g(f(x)));
    let chained = x2.fmap(f).fmap(g);

    assert_eq!(composed, chained);
}

#[test]
fn test_functor_composition_law_result() {
    let f = |x: i32| x + 1;
    let g = |x: i32| x * 2;

    let x1: Result<i32, String> = Ok(5);
    let x2: Result<i32, String> = Ok(5);

    let composed = x1.fmap(|x| g(f(x)));
    let chained = x2.fmap(f).fmap(g);

    assert_eq!(composed, chained);
}
