use crate::schema;
use crate::validation::validate::Validate;
use serde_json::json;

#[test]
fn test_validate_valid_string() {
    let schema = schema!({
        "type": "string",
        "value": "test"
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_string() {
    let schema = schema!({
        "type": "string",
        "value": 1
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}

#[test]
fn test_validate_valid_object() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": "test"
            }
        }
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_object() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": 1
            }
        }
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "properties.test");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}

#[test]
fn test_validate_valid_array() {
    let schema = schema!({
        "type": "array",
        "length": 1,
        "items": {
            "type": "string",
            "value": "test"
        }
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_array() {
    let schema = schema!({
        "type": "array",
        "length": 1,
        "items": {
            "type": "string",
            "value": 1
        }
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "items");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}

#[test]
fn test_validate_valid_any_of() {
    let schema = schema!({
        "type": "anyOf",
        "values": [
            {
                "type": "string",
                "value": "test"
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_any_of() {
    let schema = schema!({
        "type": "anyOf",
        "values": [
            {
                "type": "string",
                "value": 1
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "values.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}

#[test]
fn test_validate_valid_flatten() {
    let schema = schema!({
        "type": "flatten",
        "values": [
            {
                "type": "object",
                "properties": {
                    "test": {
                        "type": "string",
                        "value": "test"
                    }
                }
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_flatten() {
    let schema = schema!({
        "type": "flatten",
        "values": [
            {
                "type": "object",
                "properties": {
                    "test": {
                        "type": "string",
                        "value": 1
                    }
                }
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "values.0.properties.test");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}

#[test]
fn test_validate_invalid_file() {
    let schema = schema!({
        "type": "file",
        "path": "invalid.json"
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(
        error[0].message,
        "Failed to open file at path 'invalid.json'"
    );
    assert_eq!(error[0].path, "");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!("invalid.json")
    );
}

#[test]
fn test_validate_multiple_errors() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": 1
            },
            "test2": {
                "type": "string",
                "value": 2
            }
        }
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 2);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(error[0].path, "properties.test");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
    assert_eq!(error[1].message, "Failed to parse schema");
    assert_eq!(error[1].path, "properties.test2");
    assert_eq!(
        error[1].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 2})
    );
}

#[test]
fn test_validate_complex() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "array",
                "length": 1,
                "items": {
                    "type": "anyOf",
                    "values": [
                        {
                            "type": "object",
                            "properties": {
                                "test": {
                                    "type": "string",
                                    "value": 1
                                }
                            }
                        }
                    ]
                }
            }
        }
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse schema");
    assert_eq!(
        error[0].path,
        "properties.test.items.values.0.properties.test"
    );
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({"type": "string", "value": 1})
    );
}
