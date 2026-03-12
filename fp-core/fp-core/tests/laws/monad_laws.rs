// Monad Laws Tests
// These tests verify that Monad implementations satisfy the mathematical laws:
// 1. Left Identity: pure(a).chain(f) == f(a)
// 2. Right Identity: m.chain(pure) == m
// 3. Associativity: m.chain(f).chain(g) == m.chain(|x| f(x).chain(g))

use fp_core::chain::Chain;

// Left Identity Law Tests
#[test]
fn monad_law_left_identity_option() {
    let a = 42;
    let f = |x: i32| Some(x * 2);

    let left = Some(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

#[test]
fn monad_law_left_identity_option_none_result() {
    let a = 42;
    let f = |_: i32| None;

    let left = Some(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

#[test]
fn monad_law_left_identity_result() {
    let a = 42;
    let f = |x: i32| Ok::<i32, String>(x * 2);

    let left: Result<i32, String> = Ok(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

#[test]
fn monad_law_left_identity_result_err_result() {
    let a = 42;
    let f = |_: i32| Err::<i32, String>("error".to_string());

    let left: Result<i32, String> = Ok(a).chain(f);
    let right = f(a);

    assert_eq!(left, right);
}

// Right Identity Law Tests
#[test]
fn monad_law_right_identity_option_some() {
    let m = Some(42);
    let result = m.chain(|x| Some(x));
    assert_eq!(result, m);
}

#[test]
fn monad_law_right_identity_option_none() {
    let m: Option<i32> = None;
    let result = m.chain(|x| Some(x));
    assert_eq!(result, m);
}

#[test]
fn monad_law_right_identity_result_ok() {
    let m: Result<i32, String> = Ok(42);
    let result = m.chain(|x| Ok(x));
    assert_eq!(result, m);
}

#[test]
fn monad_law_right_identity_result_err() {
    let m: Result<i32, String> = Err("error".to_string());
    let result = m.chain(|x| Ok(x));
    assert_eq!(result, m);
}

// Associativity Law Tests
#[test]
fn monad_law_associativity_option() {
    let m = Some(5);
    let f = |x: i32| Some(x * 2);
    let g = |x: i32| Some(x + 1);

    let left = m.chain(f).chain(g);
    let right = Some(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn monad_law_associativity_option_with_none() {
    let m = Some(5);
    let f = |x: i32| Some(x * 2);
    let g = |_: i32| None;

    let left = m.chain(f).chain(g);
    let right = Some(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn monad_law_associativity_option_initial_none() {
    let m: Option<i32> = None;
    let f = |x: i32| Some(x * 2);
    let g = |x: i32| Some(x + 1);

    let left = m.chain(f).chain(g);
    let right: Option<i32> = None.chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn monad_law_associativity_result() {
    let m: Result<i32, String> = Ok(5);
    let f = |x: i32| Ok::<i32, String>(x * 2);
    let g = |x: i32| Ok::<i32, String>(x + 1);

    let left = m.chain(f).chain(g);
    let right: Result<i32, String> = Ok(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn monad_law_associativity_result_with_error() {
    let m: Result<i32, String> = Ok(5);
    let f = |x: i32| Ok::<i32, String>(x * 2);
    let g = |_: i32| Err::<i32, String>("error".to_string());

    let left = m.chain(f).chain(g);
    let right: Result<i32, String> = Ok(5).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

#[test]
fn monad_law_associativity_result_initial_error() {
    let m: Result<i32, String> = Err("initial error".to_string());
    let f = |x: i32| Ok::<i32, String>(x * 2);
    let g = |x: i32| Ok::<i32, String>(x + 1);

    let left = m.chain(f).chain(g);
    let right: Result<i32, String> = Err("initial error".to_string()).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}

// Complex associativity tests
#[test]
fn monad_law_associativity_three_functions() {
    let m = Some(2);
    let f = |x: i32| Some(x + 1);
    let g = |x: i32| Some(x * 2);
    let h = |x: i32| Some(x - 1);

    // (m >>= f) >>= g) >>= h
    let left = m.chain(f).chain(g).chain(h);

    // m >>= (\x -> (f x >>= g) >>= h)
    let right = Some(2).chain(|x| f(x).chain(g).chain(h));

    assert_eq!(left, right);
}

// Property: Monads compose properly
#[test]
fn monad_composition_preserves_order() {
    let m = Some(1);

    let result = m
        .chain(|x| Some(x + 1))
        .chain(|x| Some(x * 2))
        .chain(|x| Some(x - 1));

    // ((1 + 1) * 2) - 1 = (2 * 2) - 1 = 4 - 1 = 3
    assert_eq!(result, Some(3));
}

#[test]
fn monad_short_circuits_on_none() {
    let mut call_count = 0;

    let _result: Option<i32> = Some(1)
        .chain(|x| Some(x + 1))
        .chain(|_| None)
        .chain(|x| {
            call_count += 1;
            Some(x * 2)
        });

    // The last chain should not be called because previous returned None
    assert_eq!(call_count, 0);
}

#[test]
fn monad_short_circuits_on_err() {
    let mut call_count = 0;

    let _result: Result<i32, String> = Ok(1)
        .chain(|x| Ok(x + 1))
        .chain(|_| Err("error".to_string()))
        .chain(|x| {
            call_count += 1;
            Ok(x * 2)
        });

    // The last chain should not be called because previous returned Err
    assert_eq!(call_count, 0);
}

// Relationship between pure and chain
#[test]
fn monad_pure_and_chain_interaction() {
    let value = 42;

    // pure followed by chain should be the same as just applying the function
    let chained = Some(value).chain(|x| Some(x * 2));
    let direct = Some(value * 2);

    assert_eq!(chained, direct);
}

#[test]
fn monad_laws_with_type_changes() {
    let m = Some(42);
    let f = |x: i32| Some(x.to_string());
    let g = |s: String| Some(s.len());

    // Left associativity with type changes
    let left = m.chain(f).chain(g);
    let right = Some(42).chain(|x| f(x).chain(g));

    assert_eq!(left, right);
}
