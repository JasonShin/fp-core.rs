#[test]
fn hof() {
    let filter = |predicate: fn(&i32) -> bool, xs: Vec<i32>| {
        // A good Reddit post on how Filter works https://www.reddit.com/r/rust/comments/3bmua6/can_someone_help_me_understand_stditerfilter/
        xs.into_iter().filter(predicate).collect::<Vec<i32>>()
    };

    let is_even = |x: &i32| x % 2 == 0;

    let result = filter(is_even, vec![1, 2, 3, 4, 5, 6]);

    assert_eq!(result, vec![2, 4, 6]);
}
