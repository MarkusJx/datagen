use crate::schema;
use crate::validation::validate::Validate;
use serde_json::json;

#[test]
fn test_validate_valid_transform() {
    let schema = schema!({
        "type": "string",
        "value": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": "^test$"
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_ok());
}

#[test]
fn test_validate_invalid_transform() {
    let schema = schema!({
        "type": "string",
        "value": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_complex_transform() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "array",
                "length": 1,
                "items": {
                    "type": "string",
                    "value": "test",
                    "transform": [
                        {
                            "type": "regexFilter",
                            "pattern": 123
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
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "properties.test.items.transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_number_transform() {
    let schema = schema!({
        "type": "number",
        "value": 1,
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_boolean_transform() {
    let schema = schema!({
        "type": "bool",
        "value": true,
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_array_transform() {
    let schema = schema!({
        "type": "array",
        "length": 1,
        "items": {
            "type": "string",
            "value": "test"
        },
        "transform": [
                {
                    "type": "regexFilter",
                    "pattern": 123
                }
            ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1, "{error}");
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_object_transform() {
    let schema = schema!({
        "type": "object",
        "properties": {
            "test": {
                "type": "string",
                "value": "test"
            }
        },
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_integer_transform() {
    let schema = schema!({
        "type": "integer",
        "value": 1,
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_counter_transform() {
    let schema = schema!({
        "type": "counter",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_any_of_transform() {
    let schema = schema!({
        "type": "anyOf",
        "values": [
            {
                "type": "string",
                "value": "test"
            }
        ],
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_reference_transform() {
    let schema = schema!({
        "type": "reference",
        "reference": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_flatten_transform() {
    let schema = schema!({
        "type": "flatten",
        "values": [],
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 1);
    assert_eq!(error[0].message, "Failed to parse transform schema");
    assert_eq!(error[0].path, "transform.0");
    assert_eq!(
        error[0].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}

#[test]
fn test_validate_invalid_file_transform() {
    let schema = schema!({
        "type": "file",
        "path": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": 123
            }
        ]
    });

    let result = schema.validate_root();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.len(), 2);
    assert_eq!(error[0].message, "Failed to open file at path 'test'");
    assert_eq!(error[0].path, "");
    assert_eq!(error[0].invalid_value.clone().unwrap(), json!("test"));
    assert_eq!(error[1].message, "Failed to parse transform schema");
    assert_eq!(error[1].path, "transform.0");
    assert_eq!(
        error[1].invalid_value.clone().unwrap(),
        json!({
            "type": "regexFilter",
            "pattern": 123
        })
    );
}
