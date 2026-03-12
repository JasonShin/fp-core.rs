use fp_core::empty::Empty;
use fp_core::semigroup::Semigroup;

// Semigroup tests
#[test]
fn test_semigroup_i32_combine() {
    let a = 5;
    let b = 10;
    let result = a.combine(b);
    assert_eq!(result, 15);
}

#[test]
fn test_semigroup_i64_combine() {
    let a: i64 = 100;
    let b: i64 = 200;
    let result = a.combine(b);
    assert_eq!(result, 300);
}

#[test]
fn test_semigroup_f32_combine() {
    let a: f32 = 1.5;
    let b: f32 = 2.5;
    let result = a.combine(b);
    assert_eq!(result, 4.0);
}

#[test]
fn test_semigroup_f64_combine() {
    let a: f64 = 3.14;
    let b: f64 = 2.86;
    let result = a.combine(b);
    assert_eq!(result, 6.0);
}

#[test]
fn test_semigroup_string_combine() {
    let a = "hello".to_string();
    let b = " world".to_string();
    let result = a.combine(b);
    assert_eq!(result, "hello world");
}

#[test]
fn test_semigroup_vec_combine() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let result = a.combine(b);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_semigroup_vec_empty_combine() {
    let a: Vec<i32> = vec![];
    let b = vec![1, 2, 3];
    let result = a.combine(b);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_semigroup_string_empty_combine() {
    let a = "".to_string();
    let b = "hello".to_string();
    let result = a.combine(b);
    assert_eq!(result, "hello");
}

// Empty tests
#[test]
fn test_empty_i32() {
    let result = i32::empty();
    assert_eq!(result, 0);
}

#[test]
fn test_empty_i64() {
    let result = i64::empty();
    assert_eq!(result, 0);
}

#[test]
fn test_empty_u32() {
    let result = u32::empty();
    assert_eq!(result, 0);
}

#[test]
fn test_empty_f32() {
    let result = f32::empty();
    assert_eq!(result, 0.0);
}

#[test]
fn test_empty_f64() {
    let result = f64::empty();
    assert_eq!(result, 0.0);
}

#[test]
fn test_empty_string() {
    let result = String::empty();
    assert_eq!(result, "");
}

#[test]
fn test_empty_vec() {
    let result: Vec<i32> = Vec::empty();
    assert_eq!(result, vec![]);
}

// Monoid laws tests
#[test]
fn test_monoid_left_identity_i32() {
    // Left identity: empty().combine(x) == x
    let x = 42;
    let result = i32::empty().combine(x);
    assert_eq!(result, x);
}

#[test]
fn test_monoid_right_identity_i32() {
    // Right identity: x.combine(empty()) == x
    let x = 42;
    let result = x.combine(i32::empty());
    assert_eq!(result, x);
}

#[test]
fn test_monoid_left_identity_string() {
    let x = "hello".to_string();
    let result = String::empty().combine(x.clone());
    assert_eq!(result, x);
}

#[test]
fn test_monoid_right_identity_string() {
    let x = "hello".to_string();
    let result = x.clone().combine(String::empty());
    assert_eq!(result, x);
}

#[test]
fn test_monoid_left_identity_vec() {
    let x = vec![1, 2, 3];
    let result = Vec::<i32>::empty().combine(x.clone());
    assert_eq!(result, x);
}

#[test]
fn test_monoid_right_identity_vec() {
    let x = vec![1, 2, 3];
    let result = x.clone().combine(Vec::<i32>::empty());
    assert_eq!(result, x);
}

#[test]
fn test_semigroup_associativity_i32() {
    // Associativity: (a.combine(b)).combine(c) == a.combine(b.combine(c))
    let a = 5;
    let b = 10;
    let c = 15;

    let left = a.combine(b).combine(c);
    let right = a.combine(b.combine(c));

    assert_eq!(left, right);
}

#[test]
fn test_semigroup_associativity_string() {
    let a = "hello".to_string();
    let b = " ".to_string();
    let c = "world".to_string();

    let left = a.clone().combine(b.clone()).combine(c.clone());
    let right = a.combine(b.combine(c));

    assert_eq!(left, right);
}

#[test]
fn test_semigroup_associativity_vec() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let c = vec![5, 6];

    let left = a.clone().combine(b.clone()).combine(c.clone());
    let right = a.combine(b.combine(c));

    assert_eq!(left, right);
}

#[test]
fn test_monoid_multiple_combines_i32() {
    let values = vec![1, 2, 3, 4, 5];
    let result = values
        .into_iter()
        .fold(i32::empty(), |acc, x| acc.combine(x));
    assert_eq!(result, 15);
}

#[test]
fn test_monoid_multiple_combines_string() {
    let values = vec!["hello", " ", "world", "!"];
    let result = values.into_iter().fold(String::empty(), |acc, x| {
        acc.combine(x.to_string())
    });
    assert_eq!(result, "hello world!");
}

#[test]
fn test_monoid_multiple_combines_vec() {
    let values = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
    let result = values
        .into_iter()
        .fold(Vec::<i32>::empty(), |acc, x| acc.combine(x));
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_monoid_empty_accumulation() {
    // Combining only empty values should yield empty
    let result = String::empty()
        .combine(String::empty())
        .combine(String::empty());
    assert_eq!(result, String::empty());
}

#[test]
fn test_semigroup_numeric_types() {
    // Test various numeric types
    assert_eq!(1u8.combine(2u8), 3u8);
    assert_eq!(1u16.combine(2u16), 3u16);
    assert_eq!(1u32.combine(2u32), 3u32);
    assert_eq!(1u64.combine(2u64), 3u64);
    assert_eq!(1usize.combine(2usize), 3usize);
    assert_eq!(1i8.combine(2i8), 3i8);
    assert_eq!(1i16.combine(2i16), 3i16);
}

#[test]
fn test_vec_semigroup_with_strings() {
    let a = vec!["hello".to_string()];
    let b = vec!["world".to_string()];
    let result = a.combine(b);
    assert_eq!(result, vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn test_practical_monoid_sum() {
    // Practical example: summing a list
    fn sum(numbers: Vec<i32>) -> i32 {
        numbers
            .into_iter()
            .fold(i32::empty(), |acc, x| acc.combine(x))
    }

    assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(sum(vec![]), 0);
}

#[test]
fn test_practical_monoid_concat() {
    // Practical example: concatenating strings
    fn concat(strings: Vec<String>) -> String {
        strings
            .into_iter()
            .fold(String::empty(), |acc, x| acc.combine(x))
    }

    assert_eq!(
        concat(vec![
            "hello".to_string(),
            " ".to_string(),
            "world".to_string()
        ]),
        "hello world"
    );
    assert_eq!(concat(vec![]), "");
}
