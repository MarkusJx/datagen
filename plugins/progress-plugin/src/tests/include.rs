use crate::{PluginWithSchemaResult, ProgressPlugin};
use datagen_rs::util::helpers::{generate_random_data, read_schema};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[test]
fn test_include() {
    let current = Arc::new(AtomicUsize::new(0));
    let total = Arc::new(AtomicUsize::new(0));

    let current_copy = current.clone();
    let total_copy = total.clone();
    let PluginWithSchemaResult { schema, plugins } = ProgressPlugin::with_schema(
        read_schema("src/tests/simple.json").unwrap(),
        move |current, total| {
            current_copy.store(current, std::sync::atomic::Ordering::Relaxed);
            total_copy.store(total, std::sync::atomic::Ordering::Relaxed);
        },
    )
    .unwrap();

    let generated = generate_random_data(schema, Some(plugins)).unwrap();
    assert_eq!(generated, r#"{"array":["foo","foo","foo"]}"#);
    assert_eq!(current.load(std::sync::atomic::Ordering::Relaxed), 5);
    assert_eq!(total.load(std::sync::atomic::Ordering::Relaxed), 5);
}

#[test]
fn test_include_nested() {
    let current = Arc::new(AtomicUsize::new(0));
    let total = Arc::new(AtomicUsize::new(0));

    let current_copy = current.clone();
    let total_copy = total.clone();
    let PluginWithSchemaResult { schema, plugins } = ProgressPlugin::with_schema(
        read_schema("src/tests/nested.json").unwrap(),
        move |current, total| {
            current_copy.store(current, std::sync::atomic::Ordering::Relaxed);
            total_copy.store(total, std::sync::atomic::Ordering::Relaxed);
        },
    )
    .unwrap();

    let generated = generate_random_data(schema, Some(plugins)).unwrap();
    assert_eq!(generated, r#"{"inner":{"array":["foo","foo","foo"]}}"#);
    assert_eq!(current.load(std::sync::atomic::Ordering::Relaxed), 6);
    assert_eq!(total.load(std::sync::atomic::Ordering::Relaxed), 6);
}
