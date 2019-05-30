use std::time::SystemTime;

#[test]
fn side_effects() {
    let now = SystemTime::now();
    println!("{:?}", now);
}