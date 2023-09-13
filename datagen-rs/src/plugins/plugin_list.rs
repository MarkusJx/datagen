#[cfg(feature = "plugin")]
use crate::generate::generated_schema::generate::IntoGeneratedArc;
#[cfg(feature = "plugin")]
use crate::plugins::imported_plugin::ImportedPlugin;
use crate::plugins::plugin::Plugin;
#[cfg(feature = "plugin")]
use crate::schema::any::Any;
#[cfg(feature = "plugin")]
use crate::schema::any_value::AnyValue;
#[cfg(feature = "plugin")]
use crate::schema::array::Array;
#[cfg(feature = "plugin")]
use crate::schema::flatten::{Flatten, FlattenableValue};
#[cfg(feature = "plugin")]
use crate::schema::object::Object;
#[cfg(feature = "plugin")]
use crate::schema::schema_definition::PluginInitArgs;
#[cfg(feature = "plugin")]
use crate::schema::schema_definition::Schema;
#[cfg(feature = "plugin")]
use crate::schema::schema_definition::Serializer;
#[cfg(feature = "plugin")]
use crate::schema::transform::Transform;
use crate::util::types::Result;
#[cfg(feature = "plugin")]
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
#[cfg(feature = "plugin")]
use std::sync::Arc;

#[derive(Debug)]
pub struct PluginList {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginList {
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    #[cfg(feature = "plugin")]
    pub fn from_schema(
        schema: &Schema,
        additional_plugins: Option<HashMap<String, Box<dyn Plugin>>>,
    ) -> Result<Arc<Self>> {
        let additional = additional_plugins.unwrap_or_default();
        let mut plugins = schema
            .options
            .as_ref()
            .and_then(|o| o.plugins.as_ref())
            .map(|p| {
                p.clone()
                    .into_iter()
                    .filter(|(name, _)| !additional.contains_key(name))
                    .map(|(name, args)| match args {
                        PluginInitArgs::Args { path, args } => Self::map_plugin(
                            name,
                            args.clone().unwrap_or_default(),
                            Some(path.clone()),
                        ),
                        PluginInitArgs::Value(v) => Self::map_plugin(name, v, None),
                    })
                    .collect::<Result<HashMap<_, _>>>()
            })
            .map_or(Ok(None), |v| v.map(Some))?
            .unwrap_or_default();

        plugins.extend(additional);
        Self::add_plugins(&mut plugins, schema, Self::find_transformers)?;
        Self::add_plugins(&mut plugins, schema, Self::find_generators)?;

        if let Serializer::Plugin { plugin_name, .. } = schema
            .options
            .as_ref()
            .and_then(|o| o.serializer.as_ref())
            .unwrap_or_default()
        {
            if !plugins.contains_key(plugin_name) {
                plugins.insert(
                    plugin_name.clone(),
                    Self::map_plugin(plugin_name.clone(), Value::Null, None)?.1,
                );
            }
        }

        Ok(Self { plugins }.into())
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
                .map(|p| Self::map_plugin(p, Value::Null, None))
                .collect::<Result<HashMap<_, _>>>()?,
        );

        Ok(())
    }

    #[cfg(feature = "plugin")]
    fn map_plugin(
        name: String,
        args: Value,
        path: Option<String>,
    ) -> Result<(String, Box<dyn Plugin>)> {
        Ok((
            name.clone(),
            Box::new(
                ImportedPlugin::load(path.unwrap_or(name.clone()), args).map_err(
                    |e| -> Box<dyn Error> { format!("Failed to load plugin '{name}': {e}").into() },
                )?,
            ),
        ))
    }

    #[cfg(feature = "plugin")]
    fn transformers_to_vec(transform: &[Transform], loaded: &[String]) -> Vec<String> {
        transform
            .iter()
            .filter_map(|t| match t {
                Transform::Plugin(plugin) => Some(plugin.name.clone()),
                _ => None,
            })
            .filter(|name| !loaded.contains(name))
            .collect()
    }

    #[cfg(feature = "plugin")]
    fn find_object_transformers(object: &Object) -> Vec<String> {
        let mut props = object
            .properties
            .iter()
            .flat_map(|(_, val)| Self::find_transformers(val))
            .collect::<Vec<String>>();
        if let Some(transform) = &object.transform {
            props.extend(Self::transformers_to_vec(transform, &props))
        }

        props
    }

    #[cfg(feature = "plugin")]
    fn find_array_transformers(array: &Array) -> Vec<String> {
        let mut props = Self::find_transformers(&array.items);
        if let Some(transform) = &array.transform {
            props.extend(Self::transformers_to_vec(transform, &props));
        }

        props
    }

    #[cfg(feature = "plugin")]
    fn find_flatten_transformers(flatten: &Flatten) -> Vec<String> {
        let mut props = flatten
            .values
            .iter()
            .flat_map(|val| match val {
                FlattenableValue::Object(obj) => Self::find_object_transformers(obj),
                FlattenableValue::Reference(reference) => Self::map_transform(reference),
                FlattenableValue::Plugin(gen) => Self::map_transform(gen),
                FlattenableValue::Array(array) => Self::find_array_transformers(array),
            })
            .collect::<Vec<String>>();
        if let Some(transform) = &flatten.transform {
            props.extend(Self::transformers_to_vec(transform, &props));
        }

        props
    }

    #[cfg(feature = "plugin")]
    fn map_transform<T: IntoGeneratedArc>(val: &T) -> Vec<String> {
        val.get_transform()
            .map(|t| Self::transformers_to_vec(&t, &[]))
            .unwrap_or_default()
    }

    /// I am Bumblebee, I am Bumblebee, I am Bumblebee
    #[cfg(feature = "plugin")]
    fn find_transformers(any: &AnyValue) -> Vec<String> {
        match any {
            AnyValue::Any(any) => match any {
                Any::Object(object) => Self::find_object_transformers(object),
                Any::Array(array) => Self::find_array_transformers(array),
                Any::Flatten(flatten) => Self::find_flatten_transformers(flatten),
                rest => match rest {
                    Any::String(str) => str.get_transform(),
                    Any::AnyOf(any_of) => any_of.get_transform(),
                    Any::Reference(reference) => reference.get_transform(),
                    Any::Integer(integer) => IntoGeneratedArc::get_transform(integer),
                    Any::Number(number) => IntoGeneratedArc::get_transform(number),
                    Any::Counter(counter) => IntoGeneratedArc::get_transform(counter),
                    Any::Bool(boolean) => IntoGeneratedArc::get_transform(boolean),
                    Any::Plugin(plugin) => plugin.get_transform(),
                    Any::File(file) => IntoGeneratedArc::get_transform(file),
                    Any::Object(_) => panic!("Object should be handled above"),
                    Any::Array(_) => panic!("Array should be handled above"),
                    Any::Flatten(_) => panic!("Flatten should be handled above"),
                }
                .map(|t| Self::transformers_to_vec(&t, &[]))
                .unwrap_or_default(),
            },
            _ => vec![],
        }
    }

    #[cfg(feature = "plugin")]
    fn find_generators(any: &AnyValue) -> Vec<String> {
        match any {
            AnyValue::Any(Any::Plugin(gen)) => vec![gen.plugin_name.clone()],
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
