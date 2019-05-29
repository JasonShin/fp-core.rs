#[test]
fn purity() {
    let greet = |name: &str| { format!("Hi! {}", name) };

    assert_eq!("Hi! Jason", greet("Jason"));
}

#[test]
fn impure() {
    let name = "Jason";

    let greet = || -> String {
        format!("Hi! {}", name)
    };

    assert_eq!("Hi! Jason", greet());
}

#[test]
fn impure2() {
    let mut greeting = h
}
