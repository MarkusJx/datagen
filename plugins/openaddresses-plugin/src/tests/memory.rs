use crate::tests::{generate_random, get_by_hash, TestAddress, ADDR_FILE_NAME};
use crate::OpenAddressesPlugin;
use datagen_rs::plugins::plugin::{PluginConstructor, PluginOptions};
use serde_json::json;

fn create(file: Option<&str>) -> anyhow::Result<OpenAddressesPlugin> {
    OpenAddressesPlugin::new(
        json!({
            "files": file.unwrap_or(ADDR_FILE_NAME),
            "backend": {
                "type": "memory",
            }
        }),
        PluginOptions::default(),
    )
}

#[test]
fn test_generate() {
    let plugin = create(None).unwrap();

    let generated = generate_random(&plugin);
    assert_eq!(generated, get_by_hash(&generated.hash));
}

#[test]
fn test_get_random() {
    let plugin = create(None).unwrap();

    let generated: TestAddress = plugin
        .backend
        .lock()
        .unwrap()
        .get_random_feature()
        .unwrap()
        .into();
    assert_eq!(generated, get_by_hash(&generated.hash));
}
