use crate::generate::generated_schema::IntoRandom;
use crate::generate_schema;

#[test]
fn test_to_string() {
    let generated = generate_schema!({
        "type": "number",
        "value": 5,
        "transform": [
            {
                "type": "toString",
                "subType": "default"
            }
        ]
    })
    .unwrap();

    assert_eq!(
        serde_json::to_string(&generated).unwrap(),
        "\"5.0\"".to_string()
    );
}

#[test]
fn test_object_to_string() {
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
                "type": "toString",
                "subType": "default"
            }
        ]
    })
    .unwrap();

    assert_eq!(
        serde_json::to_string(&generated).unwrap(),
        "\"{\\\"a\\\":5.0,\\\"b\\\":10.0}\"".to_string()
    );
}

#[test]
fn test_format() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": {
                "type": "number",
                "value": 5
            }
        },
        "transform": [
            {
                "type": "toString",
                "subType": "format",
                "format": "The number is {{a}}",
                "serializeNonStrings": false
            }
        ]
    })
    .unwrap();

    assert_eq!(
        serde_json::to_string(&generated).unwrap(),
        "\"The number is 5\"".to_string()
    );
}

#[test]
fn test_format_serialized() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": {
                "type": "object",
                "properties": {
                    "b": {
                        "type": "number",
                        "value": 5
                    }
                }
            }
        },
        "transform": [
            {
                "type": "toString",
                "subType": "format",
                "format": "The number is {{a}}",
                "serializeNonStrings": true
            }
        ]
    })
    .unwrap();

    assert_eq!(
        serde_json::to_string(&generated).unwrap(),
        "\"The number is {&quot;b&quot;:5.0}\"".to_string()
    );
}

#[test]
fn test_format_serialized_invalid_key() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": {
                "type": "number",
                "value": 5
            }
        },
        "transform": [
            {
                "type": "toString",
                "subType": "format",
                "format": "The number is {{b}}",
                "serializeNonStrings": false
            }
        ]
    });

    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "Error rendering \"template\" line 1, col 15: Variable \"b\" not found in strict mode."
            .to_string()
    );
}

#[test]
fn test_format_invalid_object() {
    let generated = generate_schema!({
        "type": "object",
        "properties": {
            "a": {
                "type": "object",
                "properties": {
                    "b": {
                        "type": "number",
                        "value": 5
                    }
                }
            }
        },
        "transform": [
            {
                "type": "toString",
                "subType": "format",
                "format": "The number is {{a}}",
                "serializeNonStrings": false
            }
        ]
    });

    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "Cannot format non-string".to_string()
    );
}

#[test]
fn test_format_non_object() {
    let generated = generate_schema!({
        "type": "number",
        "value": 5,
        "transform": [
            {
                "type": "toString",
                "subType": "format",
                "format": "The number is {{a}}",
                "serializeNonStrings": false
            }
        ]
    });

    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "Cannot format non-object".to_string()
    );
}
