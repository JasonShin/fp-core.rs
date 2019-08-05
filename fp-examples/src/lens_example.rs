use fp_core::lens::Lens;

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct PersonNameLens;

impl Lens<Person, String> for PersonNameLens {
    fn get(s: &Person) -> Option<&String> {
        Some(&s.name)
    }

    fn set(a: String, s: &Person) -> Person {
        Person { name: a }
    }
}

struct FirstLens;

impl<A> Lens<Vec<A>, A> for FirstLens {
    fn get(s: &Vec<A>) -> Option<&A> {
        s.first()
    }

    fn set(a: A, s: &Vec<A>) -> Vec<A> {
        unimplemented!()
    }
}

#[test]
fn lens_example() {
    let e1 = Person {
        name: "Jason".to_string(),
    };
    let name = PersonNameLens::get(&e1);
    let e2 = PersonNameLens::set("John".to_string(), &e1);
    let expected = Person {
        name: "John".to_string(),
    };
    let e3 = PersonNameLens::over(&e1, &|x: Option<&String>| match x {
        Some(y) => y.to_uppercase(),
        None => panic!("T_T"),
    });
    let rando = vec![1, 2];
    let e4 = FirstLens::get(&rando);

    assert_eq!(*name.unwrap(), e1.name);
    assert_eq!(e2, expected);
    assert_eq!(
        e3,
        Person {
            name: "JASON".to_string()
        }
    );
    assert_eq!(*e4.unwrap(), 1);
}
