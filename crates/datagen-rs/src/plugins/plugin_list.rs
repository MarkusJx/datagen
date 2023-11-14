#[cfg(feature = "plugin")]
use crate::generate::generated_schema::generate::IntoGeneratedArc;
#[cfg(feature = "native-plugin")]
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
use crate::schema::serializer::Serializer;
#[cfg(feature = "plugin")]
use crate::schema::transform::Transform;
use anyhow::anyhow;
#[cfg(feature = "native-plugin")]
use anyhow::Context;
#[cfg(feature = "plugin")]
use serde_json::Value;
use std::collections::HashMap;
#[cfg(feature = "plugin")]
use std::sync::Arc;

pub type PluginMap = HashMap<String, Box<dyn Plugin>>;

#[derive(Debug)]
pub struct PluginList {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginList {
    #[cfg(any(test, feature = "test"))]
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    #[cfg(feature = "plugin")]
    pub fn from_schema(
        schema: &Schema,
        additional_plugins: Option<PluginMap>,
    ) -> anyhow::Result<Arc<Self>> {
        let plugins = Self::find_and_map_plugins(schema, additional_plugins, Self::map_plugin)?;

        Ok(Self { plugins }.into())
    }

    #[cfg(feature = "plugin")]
    pub fn find_and_map_plugins<M, R>(
        schema: &Schema,
        additional_plugins: Option<HashMap<String, R>>,
        mapper: M,
    ) -> anyhow::Result<HashMap<String, R>>
    where
        M: Fn(String, Value, String) -> anyhow::Result<Option<(String, R)>>,
    {
        let additional = additional_plugins.unwrap_or_default();
        let mut plugins = schema
            .options
            .as_ref()
            .and_then(|o| o.plugins.as_ref())
            .map(|p| {
                p.clone()
                    .into_iter()
                    .filter(|(name, _)| !additional.contains_key(name))
                    .filter_map(|(name, args)| {
                        if additional.contains_key(&name) {
                            return Some(Err(anyhow!(
                                "A plugin with name '{name}' is already loaded"
                            )));
                        }

                        match args {
                            PluginInitArgs::Args { path, args } => {
                                mapper(name, args.clone().unwrap_or_default(), path.clone())
                                    .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
                            }
                            PluginInitArgs::Value(v) => mapper(name.clone(), v, name)
                                .map_or_else(|e| Some(Err(e)), |v| v.map(Ok)),
                        }
                    })
                    .collect::<anyhow::Result<HashMap<_, _>>>()
            })
            .map_or(Ok(None), |v| v.map(Some))?
            .unwrap_or_default();

        plugins.extend(additional);
        Self::add_plugins(&mut plugins, schema, Self::find_transformers, &mapper)?;
        Self::add_plugins(&mut plugins, schema, Self::find_generators, &mapper)?;

        if let Serializer::Plugin { plugin_name, .. } = schema
            .options
            .as_ref()
            .and_then(|o| o.serializer.as_ref())
            .unwrap_or_default()
        {
            if !plugins.contains_key(plugin_name) {
                if let Some(mapped) = mapper(plugin_name.clone(), Value::Null, plugin_name.clone())?
                {
                    plugins.insert(plugin_name.clone(), mapped.1);
                }
            }
        }

        Ok(plugins)
    }

    #[cfg(feature = "plugin")]
    fn add_plugins<F, M, T>(
        plugins: &mut HashMap<String, T>,
        schema: &Schema,
        func: F,
        mapper: &M,
    ) -> anyhow::Result<()>
    where
        F: Fn(&AnyValue) -> Vec<String>,
        M: Fn(String, Value, String) -> anyhow::Result<Option<(String, T)>>,
    {
        plugins.extend(
            func(&schema.value)
                .into_iter()
                .filter(|p| !plugins.contains_key(p))
                .filter_map(|p| {
                    mapper(p.clone(), Value::Null, p).map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
                })
                .collect::<anyhow::Result<HashMap<_, _>>>()?,
        );

        Ok(())
    }

    #[cfg(feature = "native-plugin")]
    fn map_plugin(
        name: String,
        args: Value,
        path: String,
    ) -> anyhow::Result<Option<(String, Box<dyn Plugin>)>> {
        Ok(Some((
            name.clone(),
            Box::new(
                ImportedPlugin::load(path, args)
                    .context(format!("Failed to load plugin '{name}'"))?,
            ),
        )))
    }

    #[cfg(all(not(feature = "native-plugin"), feature = "plugin"))]
    fn map_plugin(
        _name: String,
        _args: Value,
        _path: String,
    ) -> anyhow::Result<Option<(String, Box<dyn Plugin>)>> {
        Err(anyhow!("Native plugin support is not enabled"))
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
        let mut props = match array {
            Array::RandomArray(arr) => Self::find_transformers(&arr.items),
            Array::ArrayWithValues(arr) => arr
                .values
                .iter()
                .flat_map(Self::find_transformers)
                .collect(),
        };

        if let Some(transform) = &array.get_transform() {
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
            AnyValue::Any(any) => Self::find_transformers_in_any(any),
            _ => vec![],
        }
    }

    #[cfg(feature = "plugin")]
    fn find_transformers_in_any(any: &Any) -> Vec<String> {
        match any {
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
        }
    }

    #[cfg(feature = "plugin")]
    fn find_generators(any: &AnyValue) -> Vec<String> {
        match any {
            AnyValue::Any(any) => Self::find_generators_in_any(any),
            _ => vec![],
        }
    }

    #[cfg(feature = "plugin")]
    fn find_generators_in_any(any: &Any) -> Vec<String> {
        match any {
            Any::Plugin(gen) => vec![gen.plugin_name.clone()],
            Any::Object(obj) => obj
                .properties
                .iter()
                .flat_map(|(_, val)| Self::find_generators(val))
                .collect(),
            Any::Array(arr) => match arr.as_ref() {
                Array::RandomArray(arr) => Self::find_generators(&arr.items),
                Array::ArrayWithValues(arr) => {
                    arr.values.iter().flat_map(Self::find_generators).collect()
                }
            },
            _ => vec![],
        }
    }

    pub fn get<'a>(&'a self, key: &String) -> anyhow::Result<&'a dyn Plugin> {
        Ok(self
            .plugins
            .get(key)
            .ok_or(anyhow!("Plugin with name '{key}' is not loaded"))?
            .as_ref())
    }

    pub fn exists(&self, key: &String) -> bool {
        self.plugins.contains_key(key)
    }
}
