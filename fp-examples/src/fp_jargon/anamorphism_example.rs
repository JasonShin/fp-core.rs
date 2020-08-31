#[cfg(test)]
mod example {
    use itertools::unfold;

    #[test]
    fn anamorphism_example() {
        let count_down = unfold((8_u32, 1_u32), |state| {
            let (ref mut x1, ref mut x2) = *state;

            if *x1 == 0 {
                return None;
            }

            let next = *x1 - *x2;
            let ret = *x1;
            *x1 = next;

            Some(ret)
        });

        assert_eq!(
            count_down.collect::<Vec<u32>>(),
            vec![8, 7, 6, 5, 4, 3, 2, 1],
        );
    }
}
