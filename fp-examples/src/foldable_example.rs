#[cfg(test)]
mod example {
    use fp_core::empty::Empty;
    use fp_core::foldable::*;

    /*
    // Check out foldable.rs in fp-core
    pub trait Foldable<A, B>: HKT<A, B> {
        fn reduce<F>(b: B, ba: F) -> <Self as HKT<A, B>>::Target
        where
            F: FnOnce(B, A) -> (B, B);
    }
    */

    #[test]
    fn foldable_example() {
        let k = vec![1, 2, 3];
        let result = k.reduce(0, |i, acc| i + acc);
        assert_eq!(result, 6);
    }

    #[test]
    fn fold_map_example() {
        let k = vec![Some(1_i64), Some(2_i64), Some(3_i64), None];
        let result = fold_map(k, |&opt| opt.unwrap_or_default());
        assert_eq!(result, 6);
    }

    // Additional comprehensive foldable tests
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
    fn test_reduce_count() {
        let vec = vec![1, 2, 3, 4, 5];
        let count = vec.reduce(0, |acc, _| acc + 1);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_vec_reduce_right() {
        let vec = vec![1, 2, 3, 4];
        let result = vec.reduce_right(0, |x, acc| x + acc);
        assert_eq!(result, 10);
    }
}
