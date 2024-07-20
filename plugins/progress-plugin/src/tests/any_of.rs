use crate::{PluginWithSchemaResult, ProgressPlugin};
use datagen_rs::util::helpers::{generate_random_data, read_schema};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

#[test]
fn test_nested_any_of() {
    let current = Arc::new(AtomicUsize::new(0));
    let total = Arc::new(AtomicUsize::new(0));

    let current_copy = current.clone();
    let total_copy = total.clone();
    let PluginWithSchemaResult { schema, plugins } = ProgressPlugin::with_schema(
        read_schema("src/tests/nested-any-of.json").unwrap(),
        move |current, total| {
            current_copy.store(current, std::sync::atomic::Ordering::Relaxed);
            total_copy.store(total, std::sync::atomic::Ordering::Relaxed);
        },
    )
    .unwrap();

    let generated = generate_random_data(schema, Some(plugins)).unwrap();
    assert!(generated == "1.0" || generated == "2.0" || generated == "3.0" || generated == "4.0");
    assert_eq!(current.load(std::sync::atomic::Ordering::Relaxed), 1);
    assert_eq!(total.load(std::sync::atomic::Ordering::Relaxed), 1);
}
