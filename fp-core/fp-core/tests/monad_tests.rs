use fp_core::chain::Chain;
use fp_core::functor::Functor;

#[test]
fn test_option_chain_some() {
    let x = Some(5);
    let result = x.chain(|n| Some(n * 2));
    assert_eq!(result, Some(10));
}

#[test]
fn test_option_chain_none() {
    let x: Option<i32> = None;
    let result = x.chain(|n| Some(n * 2));
    assert_eq!(result, None);
}

#[test]
fn test_option_chain_returns_none() {
    let x = Some(5);
    let result: Option<i32> = x.chain(|_| None);
    assert_eq!(result, None);
}

#[test]
fn test_option_chain_multiple() {
    let x = Some(5);
    let result = x
        .chain(|n| Some(n * 2))
        .chain(|n| Some(n + 1))
        .chain(|n| Some(n.to_string()));
    assert_eq!(result, Some("11".to_string()));
}

#[test]
fn test_result_chain_ok() {
    let x: Result<i32, String> = Ok(10);
    let result = x.chain(|n| Ok(n * 2));
    assert_eq!(result, Ok(20));
}

#[test]
fn test_result_chain_err() {
    let x: Result<i32, String> = Err("error".to_string());
    let result = x.chain(|n: i32| Ok(n * 2));
    assert_eq!(result, Err("error".to_string()));
}

#[test]
fn test_result_chain_returns_err() {
    let x: Result<i32, String> = Ok(5);
    let result: Result<i32, String> = x.chain(|_| Err("new error".to_string()));
    assert_eq!(result, Err("new error".to_string()));
}

#[test]
fn test_result_chain_multiple() {
    let x: Result<i32, String> = Ok(5);
    let result = x
        .chain(|n| Ok(n * 2))
        .chain(|n| Ok(n + 10))
        .chain(|n| Ok(n.to_string()));
    assert_eq!(result, Ok("20".to_string()));
}

#[test]
fn test_result_chain_error_propagation() {
    let x: Result<i32, String> = Ok(5);
    let result = x
        .chain(|n| Ok(n * 2))
        .chain(|_| Err("error in middle".to_string()))
        .chain(|n: i32| Ok(n + 10));
    assert_eq!(result, Err("error in middle".to_string()));
}

#[test]
fn test_monad_left_identity_option() {
    // Left identity: pure(a).chain(f) == f(a)
    let a = 5;
    let f = |x: i32| Some(x * 2);

    let left = Some(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

#[test]
fn test_monad_left_identity_result() {
    let a = 5;
    let f = |x: i32| Ok::<i32, String>(x * 2);

    let left: Result<i32, String> = Ok(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

#[test]
fn test_monad_right_identity_option() {
    // Right identity: m.chain(pure) == m
    let m = Some(5);
    let result = m.chain(|x| Some(x));
    assert_eq!(result, Some(5));
}

#[test]
fn test_monad_right_identity_result() {
    let m: Result<i32, String> = Ok(5);
    let result = m.chain(|x| Ok(x));
    assert_eq!(result, Ok(5));
}

#[test]
fn test_monad_associativity_option() {
    // Associativity: m.chain(f).chain(g) == m.chain(|x| f(x).chain(g))
    let m = Some(5);
    let f = |x: i32| Some(x * 2);
    let g = |x: i32| Some(x + 1);

    let left = m.chain(f).chain(g);
    let right = Some(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn test_monad_associativity_result() {
    let m: Result<i32, String> = Ok(5);
    let f = |x: i32| Ok::<i32, String>(x * 2);
    let g = |x: i32| Ok::<i32, String>(x + 1);

    let left = m.chain(f).chain(g);
    let right: Result<i32, String> = Ok(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn test_chain_with_functor_option() {
    // Combining chain and fmap
    let x = Some(5);
    let result = x.fmap(|n| n * 2).chain(|n| Some(n + 10));
    assert_eq!(result, Some(20));
}

#[test]
fn test_chain_with_functor_result() {
    let x: Result<i32, String> = Ok(5);
    let result = x.fmap(|n| n * 2).chain(|n| Ok(n + 10));
    assert_eq!(result, Ok(20));
}

#[test]
fn test_chain_flattening_option() {
    // Chain flattens nested structures
    let x = Some(5);
    let result = x.chain(|n| {
        if n > 0 {
            Some(n * 2)
        } else {
            None
        }
    });
    assert_eq!(result, Some(10));
}

#[test]
fn test_chain_validation_pattern() {
    // Practical validation use case
    fn validate_positive(n: i32) -> Result<i32, String> {
        if n > 0 {
            Ok(n)
        } else {
            Err("Must be positive".to_string())
        }
    }

    fn validate_even(n: i32) -> Result<i32, String> {
        if n % 2 == 0 {
            Ok(n)
        } else {
            Err("Must be even".to_string())
        }
    }

    let valid: Result<i32, String> = Ok(10);
    let result = valid.chain(validate_positive).chain(validate_even);
    assert_eq!(result, Ok(10));

    let invalid: Result<i32, String> = Ok(5);
    let result = invalid.chain(validate_positive).chain(validate_even);
    assert_eq!(result, Err("Must be even".to_string()));
}
