trait Lens<S, A> {
    fn get(s: S) -> A;
    fn set(a: A, s: S) -> S;
}

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct PersonNameLens;

impl Lens<Person, String> for PersonNameLens {
    fn get(s: Person) -> String {
        return s.name;
    }

    fn set(a: String, s: Person) -> Person {
        return Person {
            name: a,
        }
    }
}

#[test]
fn lens_example() {
    let e1 = Person {
        name: "Jason".to_string(),
    };

    let name = PersonNameLens::get(e1.clone());
    assert_eq!(name, e1.name);
}
