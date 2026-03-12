// impl<A, B, T> HKT<A, B> for T
// where
//     T: Sized + Iterator<Item = A>,
//     U: Sized + Iterator<Item = B>,
// {
//     type URI = Self;
//     type Target = U;
// }
//
// impl<A, B, T> Functor<A, B> for T
// where
//     T: Iterator<Item = A>,
// {
//     fn fmap<F>(self, f: F) -> Self::Target
//     where
//         F: FnOnce(A) -> B,
//         A: Sized,
//         B: Sized,
//     {
//         self.map(f)
//     }
// }

#[cfg(test)]
mod example {
    use fp_core::functor::Functor;

    #[test]
    fn test_functor() {
        let z = Option::fmap(Some(1), |x| x + 1).fmap(|x| x + 1);
        assert_eq!(z, Some(3));

        // let v = vec![3, 4];
        // assert_eq!(vec![5, 6], v.iter().fmap(|x| x + 1).fmap(|x| x + 1));
    }

    #[test]
    fn test_functor_for_result() {
        let z = Result::<_, ()>::fmap(Ok(1), |x| x + 1).fmap(|x| x + 1);
        assert_eq!(z, Ok(3));
    }

    // Additional comprehensive functor tests
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

    // Functor Laws
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
}

/*
// Below is an old implementation
#[derive(Debug, PartialEq, Eq)]
pub enum Maybe<T> {
    Nothing,
    Just(T),
}

#[test]
fn functor_example_1() {
    let v: Vec<i32> = vec![1, 2, 3].into_iter().map(| x | x + 1).collect();

    assert_eq!(v, vec![2, 3, 4]);
}

pub trait Functor<'a, A, B, F>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output;
    fn fmap(&'a self, f: F) -> Self::Output;
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Maybe<A>
    where
        A: 'a,
        F: Fn(&'a A) -> B {

    type Output = Maybe<B>;
    fn fmap(&'a self, f: F) -> Maybe<B> {
        match *self {
            Maybe::Just(ref x) => Maybe::Just(f(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[test]
fn functor_example_2() {
    let just = Maybe::Just(7);
    let nothing = Maybe::fmap(&Maybe::Nothing, |x| x + 1);
    let other = Maybe::fmap(&just, |x| x + 1);
    assert_eq!(nothing, Maybe::Nothing);
    assert_eq!(other, Maybe::Just(8));
}
*/
