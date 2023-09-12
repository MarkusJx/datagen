use crate::generate::generated_schema::{GeneratedSchema, IntoRandom};
use crate::{assert_enum, generate_schema};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::BufReader;

const FILE_NAME: &str = "src/tests/schema/data.json";

#[test]
fn test_random_file() {
    let schema = generate_schema!({
        "type": "file",
        "path": FILE_NAME,
        "mode": "random"
    })
    .unwrap_or_else(|_| {
        panic!(
            "Expected file to be found in: {:?}",
            env::current_dir().unwrap()
        )
    });

    let file: Vec<Value> =
        serde_json::from_reader(BufReader::new(File::open(FILE_NAME).unwrap())).unwrap();

    let value = assert_enum!(schema.as_ref(), GeneratedSchema::Value);
    assert!(file.contains(value));
}

#[test]
fn test_sequential_file() {
    let get_value = || {
        generate_schema!({
            "type": "file",
            "path": FILE_NAME,
            "mode": "sequential"
        })
        .unwrap()
    };

    let mut file: Vec<Value> =
        serde_json::from_reader(BufReader::new(File::open(FILE_NAME).unwrap())).unwrap();
    let mut values = vec![];
    for _ in 0..file.len() + 1 {
        values.push(assert_enum!(
            get_value().as_ref().clone(),
            GeneratedSchema::Value
        ));
    }

    file.push(file.first().unwrap().clone());
    assert_eq!(values, file);
}

#[test]
fn test_file_not_found() {
    let schema = generate_schema!({
        "type": "file",
        "path": "this-file-does-not-exist.json"
    });

    assert!(schema.is_err());
    let err = schema.unwrap_err().to_string();
    assert!(
        err.contains("The system cannot find the file specified."),
        "Expected error to contain 'The system cannot find the file specified.', but got: {}",
        err
    );
}

#[test]
fn test_transform() {
    let schema = generate_schema!({
        "type": "file",
        "path": FILE_NAME,
        "mode": "random",
        "transform": [
            {
                "type": "toString",
                "subType": "default"
            }
        ]
    })
    .unwrap();

    let value = assert_enum!(schema.as_ref(), GeneratedSchema::String);
    assert!(
        value.starts_with("{\"value\":"),
        "Expected value to start with '{{\"value\":', but got: {}",
        value
    );
}
