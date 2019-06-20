use std::ops::Add;

trait Lens<S, A> {
    fn get(s: S) -> A;
    fn set(a: A, s: S) -> S;
}

#[derive(Debug, PartialEq, Clone)]
struct Street {
    num: i32,
    name: String,
}

#[derive(Debug, Clone)]
struct Address {
    city: String,
    street: Street
}

#[derive(Debug)]
struct AddressLens;

impl Lens<Address, Street> for AddressLens {
    fn get(s: Address) -> Street {
        return s.street;
    }

    fn set(a: Street, s: Address) -> Address {
       return Address {
           city: s.city,
           street: a,
       }
    }
}

#[test]
fn lens_example() {
    let e1 = Street {
        num: 12,
        name: "Cooper".to_string(),
    };
    let a1 = Address {
        city: "seoul".to_string(),
        street: e1,
    };
    AddressLens::get(a1);

    assert_eq!(AddressLens::get(a1.clone()), e1.clone());
    assert_eq!(1, 1);
}
