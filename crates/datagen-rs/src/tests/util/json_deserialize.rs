use crate::util::helpers::generate_random_data;
use serde_json::json;

#[test]
fn test_deserialize_with_env() {
    let json = r#"{"name": "${HOME}"}"#;
    envmnt::set("HOME", "test");
    let result: serde_json::Value = crate::util::json_deserialize::from_str(json).unwrap();

    assert_eq!(result, serde_json::json!({"name": "test"}));
}

#[test]
fn test_deserialize_with_env_nested() {
    let json = r#"{"name": "${HOME}", "nested": {"name": "${HOME}"}}"#;
    envmnt::set("HOME", "test");
    let result: serde_json::Value = crate::util::json_deserialize::from_str(json).unwrap();

    assert_eq!(
        result,
        serde_json::json!({"name": "test", "nested": {"name": "test"}})
    );
}

#[test]
fn test_deserialize_with_env_array() {
    let json = r#"{"name": ["${HOME}"]}"#;
    envmnt::set("HOME", "test");
    let result: serde_json::Value = crate::util::json_deserialize::from_str(json).unwrap();

    assert_eq!(result, serde_json::json!({"name": ["test"]}));
}

#[test]
fn test_deserialize_schema_with_env() {
    let schema_json = json!({
       "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "value": "${VALUE}",
            },
        },
    });
    envmnt::set("VALUE", "test");
    let result = crate::util::json_deserialize::from_value(schema_json).unwrap();

    let generated = generate_random_data(result, None).unwrap();
    assert_eq!(generated, json!({"name": "test"}).to_string());
}

#[test]
fn test_deserialize_schema_with_env_nested() {
    let schema_json = json!({
       "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "value": "$VALUE",
            },
            "nested": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "value": "${VALUE}",
                    },
                },
            },
        },
    });
    envmnt::set("VALUE", "test");
    let result = crate::util::json_deserialize::from_value(schema_json).unwrap();

    let generated = generate_random_data(result, None).unwrap();
    assert_eq!(
        generated,
        json!({"name": "test", "nested": {"name": "test"}}).to_string()
    );
}

#[test]
fn test_deserialize_with_env_not_found() {
    envmnt::remove("DOES_NOT_EXIST");
    let json = json!({
        "type": "string",
        "value": "${DOES_NOT_EXIST}",
    });
    let result = crate::util::json_deserialize::from_value(json).unwrap();

    let generated = generate_random_data(result, None).unwrap();
    assert_eq!(generated, "\"${DOES_NOT_EXIST}\"".to_string(),);
}

#[test]
fn test_deserialize_with_default_value() {
    let json = json!({
        "type": "string",
        "value": "${DOES_NOT_EXIST:-default_value}",
    });
    let result = crate::util::json_deserialize::from_value(json).unwrap();

    let generated = generate_random_data(result, None).unwrap();
    assert_eq!(generated, "\"default_value\"".to_string());
}
