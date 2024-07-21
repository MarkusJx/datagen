use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
use crate::{assert_enum, generate_schema};

#[test]
fn test_remove_all_include() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": "a",
            "b": "b",
            "c": "c"
        },
        "transform": [
            {
                "type": "removeAll",
                "include": ["a", "b"]
            }
        ]
    })
    .unwrap();

    let object = assert_enum!(generated.as_ref(), GeneratedSchema::Object);
    assert_eq!(object.len(), 1);

    assert!(object.contains_key("c"));
    assert_eq!(object.get("c").unwrap().to_string(), "c");
}

#[test]
fn test_remove_all_exclude() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": "a",
            "b": "b",
            "c": "c"
        },
        "transform": [
            {
                "type": "removeAll",
                "exclude": ["a", "b"]
            }
        ]
    })
    .unwrap();

    let object = assert_enum!(generated.as_ref(), GeneratedSchema::Object);
    assert_eq!(object.len(), 2);

    assert!(object.contains_key("a"));
    assert_eq!(object.get("a").unwrap().to_string(), "a");
    assert!(object.contains_key("b"));
    assert_eq!(object.get("b").unwrap().to_string(), "b");
}
