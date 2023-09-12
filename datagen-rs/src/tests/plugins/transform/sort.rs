use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
use crate::{assert_enum, generate_schema};

#[test]
fn test_sort_array() {
    let generated = generate_schema!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "integer",
            "min": 0,
            "max": 100
        },
        "transform": [
            {
                "type": "sort"
            }
        ]
    })
    .unwrap();

    let arr = assert_enum!(generated.as_ref(), GeneratedSchema::Array);
    assert_eq!(arr.len(), 5);

    let mut prev = None;
    for item in arr.iter() {
        let item = assert_enum!(item.as_ref(), GeneratedSchema::Integer);
        if let Some(prev) = prev {
            assert!(item >= prev);
        }

        prev = Some(item);
    }
}

#[test]
fn test_sort_invalid_array() {
    let res = generate_schema!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "object",
            "properties": {}
        },
        "transform": [
            {
                "type": "sort"
            }
        ]
    });

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Cannot convert Object to comparable"
    );
}

#[test]
fn test_sort_object_array() {
    let generated = generate_schema!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "object",
            "properties": {
                "test": {
                    "type": "integer",
                    "min": 0,
                    "max": 100
                }
            }
        },
        "transform": [
            {
                "type": "sort",
                "by": "test"
            }
        ]
    })
    .unwrap();

    let arr = assert_enum!(generated.as_ref(), GeneratedSchema::Array);
    assert_eq!(arr.len(), 5);

    let mut prev = None;
    for item in arr.iter() {
        let item = assert_enum!(item.as_ref(), GeneratedSchema::Object);
        let item = assert_enum!(item.get("test").unwrap().as_ref(), GeneratedSchema::Integer);
        if let Some(prev) = prev {
            assert!(item >= prev);
        }

        prev = Some(item);
    }
}

#[test]
fn test_sort_object_array_invalid_key() {
    let res = generate_schema!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "object",
            "properties": {
                "test": {
                    "type": "integer",
                    "min": 0,
                    "max": 100
                }
            }
        },
        "transform": [
            {
                "type": "sort",
                "by": "invalid"
            }
        ]
    });

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Key 'invalid' not found in object"
    );
}

#[test]
fn test_sort_invalid_object_array() {
    let res = generate_schema!({
        "type": "array",
        "length": {
            "value": 5
        },
        "items": {
            "type": "object",
            "properties": {
                "test": {
                    "type": "object",
                    "properties": {}
                }
            }
        },
        "transform": [
            {
                "type": "sort",
                "by": "test"
            }
        ]
    });

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "Cannot convert Object to comparable"
    );
}
