#[cfg(feature = "plugin")]
use crate::generate::generated_schema::IntoGeneratedArc;
#[cfg(feature = "plugin")]
use crate::plugins::imported_plugin::ImportedPlugin;
use crate::plugins::plugin::Plugin;
#[cfg(feature = "plugin")]
use crate::schema::any::Any;
#[cfg(feature = "plugin")]
use crate::schema::any_value::AnyValue;
use crate::schema::flatten::FlattenableValue;
#[cfg(feature = "plugin")]
use crate::schema::schema_definition::Schema;
use crate::util::types::Result;
#[cfg(feature = "plugin")]
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PluginList {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginList {
    #[cfg(feature = "plugin")]
    pub fn from_schema(schema: &Schema) -> Result<Self> {
        let mut plugins = schema
            .options
            .as_ref()
            .and_then(|o| o.plugins.as_ref())
            .map(|p| {
                p.clone()
                    .into_iter()
                    .map(|(n, v)| Self::map_plugin(n, v))
                    .collect::<Result<HashMap<_, _>>>()
            })
            .map_or(Ok(None), |v| v.map(Some))?
            .unwrap_or_default();

        Self::add_plugins(&mut plugins, schema, Self::find_transformers)?;
        Self::add_plugins(&mut plugins, schema, Self::find_generators)?;

        Ok(Self { plugins })
    }

    #[cfg(feature = "plugin")]
    fn add_plugins<F>(
        plugins: &mut HashMap<String, Box<dyn Plugin>>,
        schema: &Schema,
        func: F,
    ) -> Result<()>
    where
        F: Fn(&AnyValue) -> Vec<String>,
    {
        plugins.extend(
            func(&schema.value)
                .into_iter()
                .filter(|p| !plugins.contains_key(p))
                .map(|p| Self::map_plugin(p, Value::Null))
                .collect::<Result<HashMap<_, _>>>()?,
        );

        Ok(())
    }

    #[cfg(feature = "plugin")]
    fn map_plugin(name: String, args: Value) -> Result<(String, Box<dyn Plugin>)> {
        Ok((name.clone(), Box::new(ImportedPlugin::load(name, args)?)))
    }

    /// I am Bumblebee, I am Bumblebee, I am Bumblebee
    #[cfg(feature = "plugin")]
    fn find_transformers(any: &AnyValue) -> Vec<String> {
        match any {
            AnyValue::Any(any) => match any {
                Any::Object(object) => {
                    let mut props = object
                        .properties
                        .iter()
                        .flat_map(|(_, val)| Self::find_transformers(val))
                        .collect::<Vec<String>>();
                    if let Some(transform) = &object.transform {
                        props.push(transform.name.clone())
                    }

                    props
                }
                Any::Array(array) => {
                    let mut props = Self::find_transformers(&array.items);
                    if let Some(transform) = &array.transform {
                        props.push(transform.name.clone())
                    }

                    props
                }
                Any::Flatten(flatten) => {
                    let mut props = flatten
                        .values
                        .iter()
                        .flat_map(|val| match val {
                            FlattenableValue::Object(obj) => obj
                                .properties
                                .iter()
                                .flat_map(|(_, val)| Self::find_transformers(val))
                                .collect::<Vec<_>>(),
                            FlattenableValue::Reference(reference) => reference
                                .transform
                                .as_ref()
                                .map(|t| vec![t.name.clone()])
                                .unwrap_or_default(),
                            FlattenableValue::Generator(generator) => generator
                                .transform
                                .as_ref()
                                .map(|t| vec![t.name.clone()])
                                .unwrap_or_default(),
                        })
                        .collect::<Vec<String>>();
                    if let Some(transform) = &flatten.transform {
                        props.push(transform.name.clone())
                    }

                    props
                }
                rest => match rest {
                    Any::String(str) => str.get_transform(),
                    Any::AnyOf(any_of) => any_of.get_transform(),
                    Any::Reference(reference) => reference.get_transform(),
                    Any::Integer(integer) => IntoGeneratedArc::get_transform(integer),
                    Any::Number(number) => IntoGeneratedArc::get_transform(number),
                    Any::Bool(boolean) => IntoGeneratedArc::get_transform(boolean),
                    Any::Generator(generator) => generator.get_transform(),
                    Any::Object(_) => panic!("Object should be handled above"),
                    Any::Array(_) => panic!("Array should be handled above"),
                    Any::Flatten(_) => panic!("Flatten should be handled above"),
                }
                .map(|t| vec![t.name])
                .unwrap_or_default(),
            },
            _ => vec![],
        }
    }

    #[cfg(feature = "plugin")]
    fn find_generators(any: &AnyValue) -> Vec<String> {
        match any {
            AnyValue::Any(Any::Generator(gen)) => vec![gen.plugin_name.clone()],
            AnyValue::Any(Any::Object(obj)) => obj
                .properties
                .iter()
                .flat_map(|(_, val)| Self::find_generators(val))
                .collect(),
            AnyValue::Any(Any::Array(arr)) => Self::find_generators(&arr.items),
            _ => vec![],
        }
    }

    pub fn get<'a>(&'a self, key: &String) -> Result<&'a dyn Plugin> {
        Ok(self
            .plugins
            .get(key)
            .ok_or(format!("Plugin with name '{key}' is not loaded"))?
            .as_ref())
    }
}
