use std::ops::Add;

trait Lens<S, A> {
    fn get(s: S) -> A;
    fn set(a: A, s: S) -> S;
}

struct Street {
    num: i32,
    name: String,
}

struct Address {
    city: String,
    street: Street
}

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
    assert_eq!(1, 1);
}
