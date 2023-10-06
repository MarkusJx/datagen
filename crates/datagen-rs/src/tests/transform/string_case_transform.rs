use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
use crate::generate_schema;
use serde_json::json;
use std::sync::Arc;

#[test]
fn test_to_upper_case() {
    let generated = generate_schema!({
        "type": "string",
        "value": "test",
        "transform": [
            {
                "type": "toUpperCase"
            }
        ]
    })
    .unwrap();

    assert_eq!(
        generated.as_ref(),
        &GeneratedSchema::String("TEST".to_string())
    );
}

#[test]
fn test_to_lower_case() {
    let res = generate_schema!({
        "type": "string",
        "value": "TEST",
        "transform": [
            {
                "type": "toLowerCase"
            }
        ]
    })
    .unwrap();

    assert_eq!(res.as_ref(), &GeneratedSchema::String("test".to_string()));
}

fn create_object(
    convert_type: &str,
    serialize_non_strings: bool,
    recursive: bool,
) -> anyhow::Result<Arc<GeneratedSchema>> {
    generate_schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": "Test"
            }
        },
        "transform": [
            {
                "type": convert_type,
                "serializeNonStrings": serialize_non_strings,
                "recursive": recursive
            }
        ]
    })
}

#[test]
fn test_object_to_upper_case() {
    let generated = create_object("toUpperCase", true, false).unwrap();

    assert_eq!(
        "\"{\\\"TEST\\\":\\\"TEST\\\"}\"".to_string(),
        serde_json::to_string_pretty(&generated).unwrap()
    )
}

#[test]
fn test_object_to_upper_case_recursive() {
    let generated = create_object("toUpperCase", false, true).unwrap();

    assert_eq!(
        serde_json::to_string(&json!({
            "test": "TEST"
        }))
        .unwrap(),
        serde_json::to_string(&generated).unwrap()
    );
}

#[test]
fn test_object_to_upper_case_invalid() {
    let generated = create_object("toUpperCase", false, false);

    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "Cannot convert non-string value 'Object' to uppercase"
    );
}

#[test]
fn test_object_to_lower_case() {
    let generated = create_object("toLowerCase", true, false).unwrap();

    assert_eq!(
        "\"{\\\"test\\\":\\\"test\\\"}\"".to_string(),
        serde_json::to_string_pretty(&generated).unwrap()
    )
}

#[test]
fn test_object_to_lower_case_recursive() {
    let generated = create_object("toLowerCase", false, true).unwrap();

    assert_eq!(
        serde_json::to_string(&json!({
            "test": "test"
        }))
        .unwrap(),
        serde_json::to_string(&generated).unwrap()
    );
}

#[test]
fn test_object_to_lower_case_invalid() {
    let generated = create_object("toLowerCase", false, false);

    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "Cannot convert non-string value 'Object' to lowercase"
    );
}

fn create_array(
    convert_type: &str,
    serialize_non_strings: bool,
    recursive: bool,
) -> anyhow::Result<Arc<GeneratedSchema>> {
    generate_schema!({
        "type": "array",
        "length": {
            "value": 3
        },
        "items": {
            "type": "string",
            "value": "Test"
        },
        "transform": [
            {
                "type": convert_type,
                "serializeNonStrings": serialize_non_strings,
                "recursive": recursive
            }
        ]
    })
}

#[test]
fn test_array_to_upper_case() {
    let array = create_array("toUpperCase", true, false).unwrap();

    assert_eq!(
        "\"[\\\"TEST\\\",\\\"TEST\\\",\\\"TEST\\\"]\"".to_string(),
        serde_json::to_string_pretty(&array).unwrap()
    );
}

#[test]
fn test_array_to_upper_case_recursive() {
    let array = create_array("toUpperCase", false, true).unwrap();

    assert_eq!(
        serde_json::to_string(&json!(["TEST", "TEST", "TEST"])).unwrap(),
        serde_json::to_string(&array).unwrap()
    );
}

#[test]
fn test_array_to_upper_case_invalid() {
    let array = create_array("toUpperCase", false, false);

    assert!(array.is_err());
    assert_eq!(
        array.unwrap_err().to_string(),
        "Cannot convert non-string value 'Array' to uppercase"
    );
}

#[test]
fn test_array_to_lower_case() {
    let array = create_array("toLowerCase", true, false).unwrap();

    assert_eq!(
        "\"[\\\"test\\\",\\\"test\\\",\\\"test\\\"]\"".to_string(),
        serde_json::to_string_pretty(&array).unwrap()
    );
}

#[test]
fn test_array_to_lower_case_recursive() {
    let array = create_array("toLowerCase", false, true).unwrap();

    assert_eq!(
        serde_json::to_string(&json!(["test", "test", "test"])).unwrap(),
        serde_json::to_string(&array).unwrap()
    );
}

#[test]
fn test_array_to_lower_case_invalid() {
    let array = create_array("toLowerCase", false, false);

    assert!(array.is_err());
    assert_eq!(
        array.unwrap_err().to_string(),
        "Cannot convert non-string value 'Array' to lowercase"
    );
}
