use itertools::unfold;

#[test]
fn anamorphism_example() {
    let (mut x1, mut x2) = (1u32, 1u32);
    let mut fibonacci = unfold((8), move |mut x| {
        // Attempt to get the next Fibonacci number
        let next = x - 1;

        x = next;

        if &next == 0 {
            return None;
        }
        Some(next)
    });


    println!("{:?}", fibonacci.by_ref().take(8));

    itertools::assert_equal(fibonacci.by_ref().take(8),
                        vec![1, 1, 2, 3, 5, 8, 13, 21]);
    assert_eq!(1, 2);
}