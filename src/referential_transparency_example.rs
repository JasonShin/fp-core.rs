#[test]
fn referential_transparency() {

    let greet = || "Hello World!";
    let msg = greet();

    assert_eq!(msg, "Hello World!");

}
