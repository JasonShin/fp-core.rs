use fp_core::empty::Empty;
use fp_core::foldable::{fold_map, Foldable};
use fp_core::semigroup::Semigroup;

#[test]
fn test_vec_reduce() {
    let vec = vec![1, 2, 3, 4, 5];
    let result = vec.reduce(0, |acc, x| acc + x);
    assert_eq!(result, 15);
}

#[test]
fn test_vec_reduce_empty() {
    let vec: Vec<i32> = vec![];
    let result = vec.reduce(0, |acc, x| acc + x);
    assert_eq!(result, 0);
}

#[test]
fn test_vec_reduce_multiply() {
    let vec = vec![1, 2, 3, 4];
    let result = vec.reduce(1, |acc, x| acc * x);
    assert_eq!(result, 24);
}

#[test]
fn test_vec_reduce_string_concat() {
    let vec = vec!["hello", " ", "world"];
    let result = vec.reduce(String::new(), |acc, x| acc + x);
    assert_eq!(result, "hello world");
}

#[test]
fn test_vec_reduce_max() {
    let vec = vec![5, 2, 8, 1, 9, 3];
    let result = vec.reduce(i32::MIN, |acc, x| acc.max(*x));
    assert_eq!(result, 9);
}

#[test]
fn test_vec_reduce_min() {
    let vec = vec![5, 2, 8, 1, 9, 3];
    let result = vec.reduce(i32::MAX, |acc, x| acc.min(*x));
    assert_eq!(result, 1);
}

#[test]
fn test_vec_reduce_right() {
    let vec = vec![1, 2, 3, 4];
    let result = vec.reduce_right(0, |x, acc| x + acc);
    assert_eq!(result, 10);
}

#[test]
fn test_vec_reduce_right_string() {
    let vec = vec!["a", "b", "c"];
    let result = vec.reduce_right(String::new(), |x, acc| format!("{}{}", x, acc));
    assert_eq!(result, "abc");
}

#[test]
fn test_vec_reduce_right_subtraction() {
    // Demonstrates non-commutative operation
    let vec = vec![1, 2, 3];
    // reduce_right: 1 - (2 - (3 - 0)) = 1 - (2 - 3) = 1 - (-1) = 2
    let result = vec.reduce_right(0, |x, acc| x - acc);
    assert_eq!(result, 2);
}

#[test]
fn test_vec_reduce_count() {
    let vec = vec![1, 2, 3, 4, 5];
    let count = vec.reduce(0, |acc, _| acc + 1);
    assert_eq!(count, 5);
}

#[test]
fn test_vec_reduce_to_vec() {
    let vec = vec![1, 2, 3];
    let result = vec.reduce(Vec::new(), |mut acc, x| {
        acc.push(x * 2);
        acc
    });
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_fold_map_sum() {
    let vec = vec![1, 2, 3, 4, 5];
    let result: i32 = fold_map(vec, |x| *x);
    assert_eq!(result, 15);
}

#[test]
fn test_fold_map_string_concat() {
    let vec = vec![1, 2, 3];
    let result: String = fold_map(vec, |x| x.to_string());
    assert_eq!(result, "123");
}

#[test]
fn test_fold_map_with_transformation() {
    let vec = vec![1, 2, 3];
    let result: i32 = fold_map(vec, |x| x * 2);
    assert_eq!(result, 12); // 2 + 4 + 6
}

#[test]
fn test_fold_map_empty_vec() {
    let vec: Vec<i32> = vec![];
    let result: i32 = fold_map(vec, |x| *x);
    assert_eq!(result, i32::empty());
}

#[test]
fn test_fold_map_vec_of_vecs() {
    let vec = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let result: Vec<i32> = fold_map(vec, |x| x.clone());
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_reduce_filter_operation() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let evens = vec.reduce(Vec::new(), |mut acc, x| {
        if x % 2 == 0 {
            acc.push(*x);
        }
        acc
    });
    assert_eq!(evens, vec![2, 4, 6]);
}

#[test]
fn test_reduce_partition() {
    #[derive(Debug, PartialEq)]
    struct Partitioned {
        evens: Vec<i32>,
        odds: Vec<i32>,
    }

    let vec = vec![1, 2, 3, 4, 5];
    let result = vec.reduce(
        Partitioned {
            evens: vec![],
            odds: vec![],
        },
        |mut acc, x| {
            if x % 2 == 0 {
                acc.evens.push(*x);
            } else {
                acc.odds.push(*x);
            }
            acc
        },
    );

    assert_eq!(result.evens, vec![2, 4]);
    assert_eq!(result.odds, vec![1, 3, 5]);
}

#[test]
fn test_reduce_average() {
    let vec = vec![1, 2, 3, 4, 5];

    #[derive(Clone)]
    struct Accumulator {
        sum: i32,
        count: i32,
    }

    let result = vec.reduce(Accumulator { sum: 0, count: 0 }, |mut acc, x| {
        acc.sum += x;
        acc.count += 1;
        acc
    });

    let average = result.sum / result.count;
    assert_eq!(average, 3);
}

#[test]
fn test_fold_map_complex_monoid() {
    #[derive(Debug, PartialEq, Clone)]
    struct Stats {
        sum: i32,
        count: i32,
    }

    impl Semigroup for Stats {
        fn combine(self, other: Self) -> Self {
            Stats {
                sum: self.sum + other.sum,
                count: self.count + other.count,
            }
        }
    }

    impl Empty for Stats {
        fn empty() -> Self {
            Stats { sum: 0, count: 0 }
        }
    }

    use fp_core::monoid::Monoid;

    impl Monoid for Stats {}

    let vec = vec![1, 2, 3, 4, 5];
    let result: Stats = fold_map(vec, |x| Stats { sum: *x, count: 1 });

    assert_eq!(result, Stats { sum: 15, count: 5 });
}

#[test]
fn test_reduce_boolean_operations() {
    let vec = vec![true, true, true];
    let all_true = vec.reduce(true, |acc, x| acc && *x);
    assert_eq!(all_true, true);

    let vec = vec![true, false, true];
    let all_true = vec.reduce(true, |acc, x| acc && *x);
    assert_eq!(all_true, false);

    let vec = vec![false, false, false];
    let any_true = vec.reduce(false, |acc, x| acc || *x);
    assert_eq!(any_true, false);

    let vec = vec![false, true, false];
    let any_true = vec.reduce(false, |acc, x| acc || *x);
    assert_eq!(any_true, true);
}

#[test]
fn test_reduce_product() {
    let vec = vec![2, 3, 4];
    let product = vec.reduce(1, |acc, x| acc * x);
    assert_eq!(product, 24);
}

#[test]
fn test_reduce_with_index() {
    let vec = vec!["a", "b", "c"];
    let result = vec.reduce((String::new(), 0), |(mut s, idx), x| {
        s.push_str(&format!("{}:{} ", idx, x));
        (s, idx + 1)
    });
    assert_eq!(result.0.trim(), "0:a 1:b 2:c");
}
