trait Lens<S, A> {
    fn over(s: &S, f: &Fn(A) -> A) -> S {
        let result: A = f(Self::get(s));
        return Self::set(result, &s);
    }
    fn get(s: &S) -> A;
    fn set(a: A, s: &S) -> S;
}

#[derive(Debug, PartialEq, Clone)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct PersonNameLens;

impl Lens<Person, String> for PersonNameLens {
    fn get(s: &Person) -> String {
        return s.name.to_string();
    }

    fn set(a: String, s: &Person) -> Person {
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

    let name = PersonNameLens::get(&e1);
    let e2 = PersonNameLens::set("John".to_string(), &e1);
    let expected = Person {
        name: "John".to_string()
    };

    let e3 = PersonNameLens::over(&e1, &|x: String| x.to_uppercase());
    assert_eq!(name, e1.name);
    assert_eq!(e2, expected);
    assert_eq!(e3, Person { name: "JASON".to_string() });
}
