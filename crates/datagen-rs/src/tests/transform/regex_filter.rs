use crate::generate::generated_schema::IntoRandom;
use crate::generate_schema;
use crate::schema::any::Any;
use crate::schema::any_value::AnyValue;
use crate::schema::transform::Transform;
use crate::tests::util::root_schema;
use crate::transform::regex_filter::RegexFilter;
use serde_json::json;

#[test]
fn test_regex_filter_match() {
    let generated = generate_schema!({
        "type": "string",
        "value": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": "^test$"
            }
        ]
    })
    .unwrap();

    assert_eq!(
        "\"test\"",
        serde_json::to_string_pretty(&generated).unwrap()
    );
}

#[test]
fn test_regex_filter_no_match() {
    let generated = generate_schema!({
        "type": "string",
        "value": "test",
        "transform": [
            {
                "type": "regexFilter",
                "pattern": "^test2$"
            }
        ]
    })
    .unwrap();

    assert_eq!("null", serde_json::to_string_pretty(&generated).unwrap());
}

fn get_object_schema() -> AnyValue {
    let json = json!({
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
                "pattern": "test"
            }
        ]
    });

    serde_json::from_value::<AnyValue>(json).unwrap()
}

fn update_schema(regex: &str, mut any: &mut AnyValue) {
    if let AnyValue::Any(Any::Object(obj)) = &mut any {
        if let Some(transform) = &mut obj.transform {
            transform[0] = Transform::RegexFilter(RegexFilter {
                pattern: regex.to_string(),
                serialize_non_strings: Some(true),
            });
        }
    }
}

#[test]
fn test_regex_filter_serialized_value_error() {
    let schema = root_schema();
    let any = get_object_schema();

    let res = any.clone().into_random(schema.clone());
    assert!(res.is_err());
    assert_eq!(
        "Cannot filter non-string value by regex".to_string(),
        res.unwrap_err().to_string()
    );
}

#[test]
fn test_regex_filter_serialized_value_matching() {
    let schema = root_schema();
    let mut any = get_object_schema();

    update_schema("test", &mut any);
    let generated = any.clone().into_random(schema.clone()).unwrap();
    assert_eq!(
        serde_json::to_string_pretty(&json!({
            "test": "test"
        }))
        .unwrap(),
        serde_json::to_string_pretty(&generated).unwrap()
    );
}

#[test]
fn test_regex_filter_serialized_value_no_match() {
    let schema = root_schema();
    let mut any = get_object_schema();

    update_schema("test2", &mut any);
    let generated = any.into_random(schema).unwrap();
    assert_eq!("null", serde_json::to_string_pretty(&generated).unwrap());
}
