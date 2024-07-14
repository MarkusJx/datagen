use crate::generate::generated_schema::IntoRandom;
use crate::generate_schema;

#[test]
fn test_remove_array() {
    let generated = generate_schema!({
        "type": "array",
        "length": 1,
        "items": {
            "type": "number",
            "value": 5
        },
        "transform": [
            {
                "type": "randomRemove",
                "min": 1,
                "max": 1
            }
        ]
    })
    .unwrap();

    assert_eq!(serde_json::to_string(&generated).unwrap(), "[]");
}

#[test]
fn test_remove_object() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": {
                "type": "number",
                "value": 5
            },
            "b": {
                "type": "number",
                "value": 10
            }
        },
        "transform": [
            {
                "type": "randomRemove",
                "min": 1,
                "max": 1
            }
        ]
    })
    .unwrap();

    let gen = serde_json::to_string(&generated).unwrap();
    assert!(gen == "{\"a\":5.0}" || gen == "{\"b\":10.0}");
}

#[test]
fn test_remove_string() {
    let generated = generate_schema!({
        "type": "string",
        "value": "hello",
        "transform": [
            {
                "type": "randomRemove",
                "chance": 1.0
            }
        ]
    })
    .unwrap();

    assert_eq!(serde_json::to_string(&generated).unwrap(), "null");
}
