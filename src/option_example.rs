use std::collections::HashMap;

#[test]
fn option_example() {
    let mut cart = HashMap::new();
    let mut item = HashMap::new();
    item.insert("price".to_string(), 12);
    cart.insert("item".to_string(), item);

    fn get_item(cart: &HashMap<String, HashMap<String, i32>>) -> Option<&HashMap<String, i32>> {
        cart.get("item")
    }

    fn get_price(item: &HashMap<String, i32>) -> Option<&i32> {
        item.get("price")
    }

    fn get_nested_price(cart: &HashMap<String, HashMap<String, i32>>) -> Option<&i32> {
        get_item(cart).and_then(get_price)
    }

    let price = get_nested_price(&cart);

    match price {
        Some(v) => assert_eq!(v, &12),
        None => panic!("T_T"),
    }
}
