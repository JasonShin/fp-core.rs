#[test]
fn continuation() {
    let print_as_string = |num: i32| println!("Given {}", num);

    let add_one_and_continue = |num: i32, cc: fn(i32)| {
        let result = num + 1;
        cc(result)
    };

    add_one_and_continue(1, print_as_string);
}
