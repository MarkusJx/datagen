use crate::generate::datagen_context::DatagenContextRef;
use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::resolved_reference::ResolvedReference;
use crate::generate::schema_path::SchemaPath;
use crate::generate::schema_value::SchemaProperties;
use crate::plugins::abi::PluginAbi_TO;
use crate::plugins::plugin::{Plugin, PluginContainer, PluginSerializeCallback};
use crate::schema::schema_definition::SchemaOptions;
use crate::tests::plugins::context::{Context, MockContext};
use abi_stable::erased_types::TD_CanDowncast;
use serde_json::Value;
use std::sync::{Arc, Mutex};

struct TestPlugin;

impl Plugin for TestPlugin {
    fn name(&self) -> String {
        "test-plugin".into()
    }

    fn generate(
        &self,
        schema: DatagenContextRef,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        schema.child(None, "test")?;
        schema.resolve_ref("test")?;
        schema.finalize(GeneratedSchema::String("test".to_string()).into())?;
        schema.path()?;
        schema.get_plugin("test")?;
        schema.plugin_exists("test")?;
        schema.options()?;
        schema.__schema_value_properties()?;

        assert_eq!(args, Value::String("test".to_string()));
        Ok(GeneratedSchema::String("test".to_string()).into())
    }

    fn transform(
        &self,
        schema: DatagenContextRef,
        value: Arc<GeneratedSchema>,
        args: Value,
    ) -> anyhow::Result<Arc<GeneratedSchema>> {
        schema.finalize(value)?;
        assert_eq!(args, Value::String("test".to_string()));

        Ok(GeneratedSchema::String("test".to_string()).into())
    }

    fn serialize(&self, value: &Arc<GeneratedSchema>, args: Value) -> anyhow::Result<String> {
        assert_eq!(value, &GeneratedSchema::String("test".to_string()).into());
        assert_eq!(args, Value::String("test".to_string()));

        Ok("\"test\"".to_string())
    }

    fn serialize_with_progress(
        &self,
        value: &Arc<GeneratedSchema>,
        args: Value,
        callback: PluginSerializeCallback,
    ) -> anyhow::Result<String> {
        callback(1, 0)?;
        self.serialize(value, args)
    }
}

impl TestPlugin {
    fn new_container() -> Arc<dyn Plugin> {
        PluginAbi_TO::from_value(PluginContainer::new(Self), TD_CanDowncast).into()
    }
}

#[test]
fn test_name() {
    let plugin = TestPlugin::new_container();

    assert_eq!(plugin.name(), "test-plugin");
}

#[test]
fn test_generate() {
    let plugin = TestPlugin::new_container();
    let args = Value::String("test".to_string());

    let mut context = MockContext::new();
    context.expect_child().returning(|sibling, child| {
        assert!(sibling.is_none());
        assert_eq!(child, "test");
        Ok(MockContext::new().into())
    });
    context.expect_resolve_ref().returning(|reference| {
        assert_eq!(reference, "test");
        Ok(ResolvedReference::None)
    });
    context.expect_finalize().returning(|schema| {
        assert_eq!(schema, GeneratedSchema::String("test".to_string()).into());
        Ok(GeneratedSchema::String("test".to_string()).into())
    });
    context.expect_path().returning(|| Ok(SchemaPath::root()));
    context.expect_get_plugin().returning(|key| {
        assert_eq!(key, "test");
        Ok(TestPlugin::new_container())
    });
    context.expect_plugin_exists().returning(|key| {
        assert_eq!(key, "test");
        Ok(true)
    });
    context
        .expect_options()
        .returning(|| Ok(SchemaOptions::default().into()));
    context
        .expect___schema_value_properties()
        .returning(|| Ok(Arc::new(Mutex::new(SchemaProperties::new()))));

    let ctx: DatagenContextRef = context.into();
    let generated = plugin.generate(ctx.clone(), args).unwrap();

    assert_eq!("\"test\"", serde_json::to_string(&generated).unwrap());

    let mut context_cast = ctx
        .as_any()
        .downcast_ref::<Context>()
        .unwrap()
        .mock_context
        .lock()
        .unwrap();
    context_cast.checkpoint();
}

#[test]
fn test_transform() {
    let plugin = TestPlugin::new_container();
    let args = Value::String("test".to_string());

    let mut context = MockContext::new();
    context.expect_finalize().returning(|schema| {
        assert_eq!(schema, GeneratedSchema::String("test".to_string()).into());
        Ok(GeneratedSchema::String("test".to_string()).into())
    });

    let ctx: DatagenContextRef = context.into();
    let generated = GeneratedSchema::String("test".to_string()).into();
    let transformed = plugin.transform(ctx.clone(), generated, args).unwrap();

    assert_eq!("\"test\"", serde_json::to_string(&transformed).unwrap());

    let mut context_cast = ctx
        .as_any()
        .downcast_ref::<Context>()
        .unwrap()
        .mock_context
        .lock()
        .unwrap();
    context_cast.checkpoint();
}

#[test]
fn test_serialize() {
    let plugin = TestPlugin::new_container();
    let args = Value::String("test".to_string());

    let generated = GeneratedSchema::String("test".to_string()).into();
    let serialized = plugin.serialize(&generated, args).unwrap();

    assert_eq!("\"test\"", serialized);
}

#[test]
fn test_serialize_with_progress() {
    let plugin = TestPlugin::new_container();
    let args = Value::String("test".to_string());

    let generated = GeneratedSchema::String("test".to_string()).into();
    let serialized = plugin
        .serialize_with_progress(
            &generated,
            args,
            Box::new(|progress, total| {
                assert_eq!(progress, 1);
                assert_eq!(total, 0);

                Ok(())
            }),
        )
        .unwrap();

    assert_eq!("\"test\"", serialized);
}

#[test]
fn test_serialize_with_progress_and_error() {
    let plugin = TestPlugin::new_container();
    let args = Value::String("test".to_string());

    let generated = GeneratedSchema::String("test".to_string()).into();
    let res = plugin.serialize_with_progress(
        &generated,
        args,
        Box::new(|_progress, _total| Err(anyhow::anyhow!("test error"))),
    );

    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "test error");
}
