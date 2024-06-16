use crate::util::helpers::{generate_random_data, read_schema};

#[test]
fn test_include() {
    let schema = read_schema("src/tests/schema/include/simple.json").unwrap();
    let generated = generate_random_data(schema, None).unwrap();

    assert_eq!(generated, r#"{"name":"Hello, World!"}"#);
}

#[test]
fn test_include_nested() {
    let schema = read_schema("src/tests/schema/include/nested.json").unwrap();
    let generated = generate_random_data(schema, None).unwrap();

    assert_eq!(generated, r#"{"inner":{"name":"Hello, World!"}}"#);
}
