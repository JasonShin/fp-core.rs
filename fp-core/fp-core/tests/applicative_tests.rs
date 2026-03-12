use fp_core::apply::Apply;
use fp_core::pure::Pure;

#[test]
fn test_option_pure() {
    let result = Option::of(5);
    assert_eq!(result, Some(5));
}

#[test]
fn test_result_pure() {
    let result: Result<i32, String> = Result::of(5);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_option_apply() {
    let value = Some(5);
    let func = Some(Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>);
    let result = value.ap(func);
    assert_eq!(result, Some(10));
}

#[test]
fn test_option_apply_none_value() {
    let value: Option<i32> = None;
    let func = Some(Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>);
    let result = value.ap(func);
    assert_eq!(result, None);
}

#[test]
fn test_option_apply_none_func() {
    let value = Some(5);
    let func: Option<Box<dyn Fn(i32) -> i32>> = None;
    let result = value.ap(func);
    assert_eq!(result, None);
}

#[test]
fn test_result_apply_ok() {
    let value: Result<i32, String> = Ok(5);
    let func: Result<Box<dyn Fn(i32) -> i32>, String> =
        Ok(Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>);
    let result = value.ap(func);
    assert_eq!(result, Ok(10));
}

#[test]
fn test_result_apply_err_value() {
    let value: Result<i32, String> = Err("value error".to_string());
    let func: Result<Box<dyn Fn(i32) -> i32>, String> =
        Ok(Box::new(|x: i32| x * 2) as Box<dyn Fn(i32) -> i32>);
    let result = value.ap(func);
    assert_eq!(result, Err("value error".to_string()));
}

#[test]
fn test_result_apply_err_func() {
    let value: Result<i32, String> = Ok(5);
    let func: Result<Box<dyn Fn(i32) -> i32>, String> = Err("func error".to_string());
    let result = value.ap(func);
    assert_eq!(result, Err("func error".to_string()));
}

#[test]
fn test_applicative_identity_law_option() {
    // Identity: pure(id).ap(v) == v
    let v = Some(42);
    let id_func = Some(Box::new(|x: i32| x) as Box<dyn Fn(i32) -> i32>);
    let result = v.ap(id_func);
    assert_eq!(result, Some(42));
}

#[test]
fn test_applicative_identity_law_result() {
    let v: Result<i32, String> = Ok(42);
    let id_func: Result<Box<dyn Fn(i32) -> i32>, String> =
        Ok(Box::new(|x: i32| x) as Box<dyn Fn(i32) -> i32>);
    let result = v.ap(id_func);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_applicative_homomorphism_option() {
    // Homomorphism: pure(f).ap(pure(x)) == pure(f(x))
    let x = 5;
    let f = |n: i32| n * 2;

    let left = Option::of(x).ap(Some(Box::new(f) as Box<dyn Fn(i32) -> i32>));
    let right = Option::of(f(x));

    assert_eq!(left, right);
}

#[test]
fn test_applicative_homomorphism_result() {
    let x = 5;
    let f = |n: i32| n * 2;

    let left: Result<i32, String> =
        Result::of(x).ap(Ok(Box::new(f) as Box<dyn Fn(i32) -> i32>));
    let right: Result<i32, String> = Result::of(f(x));

    assert_eq!(left, right);
}

#[test]
fn test_pure_wraps_value_option() {
    let value = 42;
    let wrapped = Option::of(value);
    assert_eq!(wrapped, Some(42));
}

#[test]
fn test_pure_wraps_value_result() {
    let value = 42;
    let wrapped: Result<i32, String> = Result::of(value);
    assert_eq!(wrapped, Ok(42));
}

#[test]
fn test_pure_with_string_option() {
    let value = "hello".to_string();
    let wrapped = Option::of(value);
    assert_eq!(wrapped, Some("hello".to_string()));
}

#[test]
fn test_pure_with_string_result() {
    let value = "hello".to_string();
    let wrapped: Result<String, String> = Result::of(value);
    assert_eq!(wrapped, Ok("hello".to_string()));
}

#[test]
fn test_apply_with_type_change() {
    let value = Some(42);
    let func = Some(Box::new(|x: i32| x.to_string()) as Box<dyn Fn(i32) -> String>);
    let result = value.ap(func);
    assert_eq!(result, Some("42".to_string()));
}

#[test]
fn test_apply_result_with_type_change() {
    let value: Result<i32, String> = Ok(42);
    let func: Result<Box<dyn Fn(i32) -> String>, String> =
        Ok(Box::new(|x: i32| x.to_string()) as Box<dyn Fn(i32) -> String>);
    let result = value.ap(func);
    assert_eq!(result, Ok("42".to_string()));
}
