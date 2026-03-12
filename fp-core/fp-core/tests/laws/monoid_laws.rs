// Monoid Laws Tests
// These tests verify that Monoid implementations satisfy the mathematical laws:
// 1. Left Identity: empty().combine(x) == x
// 2. Right Identity: x.combine(empty()) == x
// 3. Associativity: (x.combine(y)).combine(z) == x.combine(y.combine(z))

use fp_core::empty::Empty;
use fp_core::semigroup::Semigroup;

// Left Identity Law Tests
#[test]
fn monoid_law_left_identity_i32() {
    let x = 42;
    assert_eq!(i32::empty().combine(x), x);
}

#[test]
fn monoid_law_left_identity_i64() {
    let x: i64 = 100;
    assert_eq!(i64::empty().combine(x), x);
}

#[test]
fn monoid_law_left_identity_f32() {
    let x: f32 = 3.14;
    assert_eq!(f32::empty().combine(x), x);
}

#[test]
fn monoid_law_left_identity_f64() {
    let x: f64 = 2.718;
    assert_eq!(f64::empty().combine(x), x);
}

#[test]
fn monoid_law_left_identity_string() {
    let x = "hello".to_string();
    assert_eq!(String::empty().combine(x.clone()), x);
}

#[test]
fn monoid_law_left_identity_vec() {
    let x = vec![1, 2, 3];
    assert_eq!(Vec::<i32>::empty().combine(x.clone()), x);
}

#[test]
fn monoid_law_left_identity_empty_string() {
    let x = String::empty();
    assert_eq!(String::empty().combine(x.clone()), x);
}

#[test]
fn monoid_law_left_identity_empty_vec() {
    let x: Vec<i32> = Vec::empty();
    assert_eq!(Vec::<i32>::empty().combine(x.clone()), x);
}

// Right Identity Law Tests
#[test]
fn monoid_law_right_identity_i32() {
    let x = 42;
    assert_eq!(x.combine(i32::empty()), x);
}

#[test]
fn monoid_law_right_identity_i64() {
    let x: i64 = 100;
    assert_eq!(x.combine(i64::empty()), x);
}

#[test]
fn monoid_law_right_identity_f32() {
    let x: f32 = 3.14;
    assert_eq!(x.combine(f32::empty()), x);
}

#[test]
fn monoid_law_right_identity_f64() {
    let x: f64 = 2.718;
    assert_eq!(x.combine(f64::empty()), x);
}

#[test]
fn monoid_law_right_identity_string() {
    let x = "hello".to_string();
    assert_eq!(x.clone().combine(String::empty()), x);
}

#[test]
fn monoid_law_right_identity_vec() {
    let x = vec![1, 2, 3];
    assert_eq!(x.clone().combine(Vec::<i32>::empty()), x);
}

// Associativity Law Tests
#[test]
fn monoid_law_associativity_i32() {
    let x = 5;
    let y = 10;
    let z = 15;

    let left = x.combine(y).combine(z);
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_i64() {
    let x: i64 = 100;
    let y: i64 = 200;
    let z: i64 = 300;

    let left = x.combine(y).combine(z);
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_f32() {
    let x: f32 = 1.5;
    let y: f32 = 2.5;
    let z: f32 = 3.0;

    let left = x.combine(y).combine(z);
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_f64() {
    let x: f64 = 1.1;
    let y: f64 = 2.2;
    let z: f64 = 3.3;

    let left = x.combine(y).combine(z);
    let right = x.combine(y.combine(z));

    // Use approximate equality for floating point
    assert!((left - right).abs() < 1e-10);
}

#[test]
fn monoid_law_associativity_string() {
    let x = "hello".to_string();
    let y = " ".to_string();
    let z = "world".to_string();

    let left = x.clone().combine(y.clone()).combine(z.clone());
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_vec() {
    let x = vec![1, 2];
    let y = vec![3, 4];
    let z = vec![5, 6];

    let left = x.clone().combine(y.clone()).combine(z.clone());
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

// Associativity with empty elements
#[test]
fn monoid_law_associativity_with_empty_i32() {
    let x = 42;
    let y = i32::empty();
    let z = 10;

    let left = x.combine(y).combine(z);
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_with_empty_string() {
    let x = "hello".to_string();
    let y = String::empty();
    let z = "world".to_string();

    let left = x.clone().combine(y.clone()).combine(z.clone());
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
}

#[test]
fn monoid_law_associativity_all_empty_string() {
    let x = String::empty();
    let y = String::empty();
    let z = String::empty();

    let left = x.clone().combine(y.clone()).combine(z.clone());
    let right = x.combine(y.combine(z));

    assert_eq!(left, right);
    assert_eq!(left, String::empty());
}

// Multiple combines
#[test]
fn monoid_law_multiple_combines_i32() {
    let values = vec![1, 2, 3, 4, 5];

    let result = values.into_iter().fold(i32::empty(), |acc, x| acc.combine(x));

    assert_eq!(result, 15);
}

#[test]
fn monoid_law_multiple_combines_string() {
    let values = vec!["a", "b", "c", "d"];

    let result = values.into_iter().fold(String::empty(), |acc, x| {
        acc.combine(x.to_string())
    });

    assert_eq!(result, "abcd");
}

#[test]
fn monoid_law_multiple_combines_vec() {
    let values = vec![vec![1], vec![2], vec![3]];

    let result = values
        .into_iter()
        .fold(Vec::<i32>::empty(), |acc, x| acc.combine(x));

    assert_eq!(result, vec![1, 2, 3]);
}

// Empty is neutral element
#[test]
fn monoid_empty_is_neutral_i32() {
    let x = 42;
    assert_eq!(i32::empty().combine(x).combine(i32::empty()), x);
}

#[test]
fn monoid_empty_is_neutral_string() {
    let x = "test".to_string();
    assert_eq!(
        String::empty().combine(x.clone()).combine(String::empty()),
        x
    );
}

#[test]
fn monoid_empty_is_neutral_vec() {
    let x = vec![1, 2, 3];
    assert_eq!(
        Vec::<i32>::empty()
            .combine(x.clone())
            .combine(Vec::<i32>::empty()),
        x
    );
}

// Property: combining multiple empties yields empty
#[test]
fn monoid_multiple_empties_i32() {
    let result = i32::empty()
        .combine(i32::empty())
        .combine(i32::empty())
        .combine(i32::empty());
    assert_eq!(result, i32::empty());
}

#[test]
fn monoid_multiple_empties_string() {
    let result = String::empty()
        .combine(String::empty())
        .combine(String::empty());
    assert_eq!(result, String::empty());
}

#[test]
fn monoid_multiple_empties_vec() {
    let result = Vec::<i32>::empty()
        .combine(Vec::<i32>::empty())
        .combine(Vec::<i32>::empty());
    assert_eq!(result, Vec::<i32>::empty());
}

// Numeric types
#[test]
fn monoid_law_all_numeric_types_left_identity() {
    assert_eq!(u8::empty().combine(5u8), 5u8);
    assert_eq!(u16::empty().combine(5u16), 5u16);
    assert_eq!(u32::empty().combine(5u32), 5u32);
    assert_eq!(u64::empty().combine(5u64), 5u64);
    assert_eq!(i8::empty().combine(5i8), 5i8);
    assert_eq!(i16::empty().combine(5i16), 5i16);
    assert_eq!(i32::empty().combine(5i32), 5i32);
    assert_eq!(i64::empty().combine(5i64), 5i64);
}

#[test]
fn monoid_law_all_numeric_types_right_identity() {
    assert_eq!(5u8.combine(u8::empty()), 5u8);
    assert_eq!(5u16.combine(u16::empty()), 5u16);
    assert_eq!(5u32.combine(u32::empty()), 5u32);
    assert_eq!(5u64.combine(u64::empty()), 5u64);
    assert_eq!(5i8.combine(i8::empty()), 5i8);
    assert_eq!(5i16.combine(i16::empty()), 5i16);
    assert_eq!(5i32.combine(i32::empty()), 5i32);
    assert_eq!(5i64.combine(i64::empty()), 5i64);
}

#[test]
fn monoid_law_numeric_associativity() {
    let x = 5u32;
    let y = 10u32;
    let z = 15u32;

    assert_eq!(x.combine(y).combine(z), x.combine(y.combine(z)));
}

// Edge cases
#[test]
fn monoid_law_large_numbers() {
    let x: i64 = i64::MAX / 2;
    let y: i64 = 100;

    assert_eq!(i64::empty().combine(x), x);
    assert_eq!(x.combine(i64::empty()), x);
    assert_eq!(x.combine(y).combine(i64::empty()), x.combine(y));
}

#[test]
fn monoid_law_negative_numbers() {
    let x = -42;
    let y = -10;
    let z = -5;

    assert_eq!(i32::empty().combine(x), x);
    assert_eq!(x.combine(i32::empty()), x);
    assert_eq!(x.combine(y).combine(z), x.combine(y.combine(z)));
}
