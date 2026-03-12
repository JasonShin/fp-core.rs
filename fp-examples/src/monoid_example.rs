#[cfg(test)]
mod example {
    use fp_core::compose::*;
    use fp_core::empty::Empty;
    use fp_core::identity::*;
    use fp_core::semigroup::Semigroup;

    fn foo(a: i32) -> i32 {
        a + 20
    }

    #[test]
    fn monoid_example() {
        let z = 1 + 1;
        let x = 1 + (2 + 3) == (1 + 2) + 3;
        let y = [vec![1, 2, 3], vec![4, 5, 6]].concat();
        let u = [vec![1, 2], vec![]].concat();
        let i = compose!(foo, identity)(1) == compose!(identity, foo)(1);
        assert_eq!(z, 2);
        assert!(x);
        assert_eq!(y, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(u, vec![1, 2]);
        assert!(i);
    }

    // Additional comprehensive monoid tests
    #[test]
    fn test_semigroup_i32_combine() {
        let a = 5;
        let b = 10;
        let result = a.combine(b);
        assert_eq!(result, 15);
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
    fn test_empty_i32() {
        let result = i32::empty();
        assert_eq!(result, 0);
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

    // Monoid Laws
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
    fn test_monoid_multiple_combines() {
        let values = vec![1, 2, 3, 4, 5];
        let result = values
            .into_iter()
            .fold(i32::empty(), |acc, x| acc.combine(x));
        assert_eq!(result, 15);
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
}
