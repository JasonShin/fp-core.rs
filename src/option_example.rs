use std::collections::HashMap;

#[test]
fn option_example() {
    let mut cart = HashMap::new();
    let mut item = HashMap::new();
    item.insert(
        "price".to_string(),
        12
    );
    cart.insert(
        "item".to_string(),
        item,
    );

    fn get_item<'a>(cart: HashMap<String, HashMap<String, i32>>) -> Option<&'a HashMap<String, i32>> {
        return cart.get("item");
    }

    fn get_price<'a>(item: &HashMap<String, i32>) -> Option<&'a i32> {
        return item.get("price");
    }

    fn get_nested_price<'a>(cart: HashMap<String, HashMap<String, i32>>) -> Option<Option<&'a i32>> {
        return get_item(cart).map(get_price);
    }
}