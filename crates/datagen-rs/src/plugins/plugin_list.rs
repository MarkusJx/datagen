#[cfg(feature = "plugin")]
use crate::generate::generated_schema::generate::IntoGeneratedArc;
#[cfg(feature = "native-plugin")]
use crate::plugins::imported_plugin::ImportedPlugin;
use crate::plugins::plugin::Plugin;
#[cfg(feature = "plugin")]
use crate::schema::any::Any;
#[cfg(feature = "plugin")]
use crate::schema::any::MaybeValidAny;
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
use crate::schema::transform::MaybeValidTransform;
#[cfg(feature = "plugin")]
use crate::schema::transform::Transform;
#[cfg(feature = "plugin")]
use crate::util::traits::GetTransform;
use anyhow::anyhow;
#[cfg(feature = "native-plugin")]
use anyhow::Context;
#[cfg(feature = "native-plugin")]
use log::debug;
#[cfg(feature = "plugin")]
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub type PluginMap = HashMap<String, Arc<dyn Plugin>>;

pub struct PluginList {
    plugins: PluginMap,
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
        schema: &mut Schema,
        additional_plugins: Option<PluginMap>,
    ) -> anyhow::Result<Arc<Self>> {
        let plugins = Self::find_and_map_plugins(schema, additional_plugins, Self::map_plugin)?;

        Ok(Self { plugins }.into())
    }

    #[cfg(feature = "plugin")]
    pub fn find_and_map_plugins<M, R>(
        schema: &mut Schema,
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
                p.iter()
                    .filter(|(name, _)| !additional.contains_key(*name))
                    .filter_map(|(name, args)| {
                        if additional.contains_key(name) {
                            return Some(Err(anyhow!(
                                "A plugin with name '{name}' is already loaded"
                            )));
                        }

                        match args {
                            PluginInitArgs::Args { path, args } => {
                                mapper(name.clone(), args.clone().unwrap_or_default(), path.clone())
                                    .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
                            }
                            PluginInitArgs::Value(v) => {
                                mapper(name.clone(), v.clone(), name.clone())
                                    .map_or_else(|e| Some(Err(e)), |v| v.map(Ok))
                            }
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
        schema: &mut Schema,
        func: F,
        mapper: &M,
    ) -> anyhow::Result<()>
    where
        F: Fn(&mut AnyValue) -> anyhow::Result<Vec<String>>,
        M: Fn(String, Value, String) -> anyhow::Result<Option<(String, T)>>,
    {
        plugins.extend(
            func(&mut schema.value)?
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
    ) -> anyhow::Result<Option<(String, Arc<dyn Plugin>)>> {
        debug!("Loading plugin '{name}' from '{path}'");
        Ok(Some((
            name.clone(),
            Arc::new(
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
    ) -> anyhow::Result<Option<(String, Arc<dyn Plugin>)>> {
        Err(anyhow!("Native plugin support is not enabled"))
    }

    #[cfg(feature = "plugin")]
    fn transformers_to_vec(transform: &[MaybeValidTransform], loaded: &[String]) -> Vec<String> {
        transform
            .iter()
            .filter_map(|t| match t {
                MaybeValidTransform::Valid(Transform::Plugin(plugin)) => Some(plugin.name.clone()),
                _ => None,
            })
            .filter(|name| !loaded.contains(name))
            .collect()
    }

    #[cfg(feature = "plugin")]
    fn find_object_transformers(object: &mut Object) -> anyhow::Result<Vec<String>> {
        let mut props = object
            .properties
            .iter_mut()
            .map(|(_, val)| Self::find_transformers(val))
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if let Some(transform) = &object.transform {
            props.extend(Self::transformers_to_vec(transform, &props))
        }

        Ok(props)
    }

    #[cfg(feature = "plugin")]
    fn find_array_transformers(array: &mut Array) -> anyhow::Result<Vec<String>> {
        let mut props = match array {
            Array::RandomArray(arr) => Self::find_transformers(&mut arr.items)?,
            Array::ArrayWithValues(arr) => arr
                .values
                .iter_mut()
                .map(Self::find_transformers)
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
        };

        if let Some(transform) = &array.get_transform() {
            props.extend(Self::transformers_to_vec(transform, &props));
        }

        Ok(props)
    }

    #[cfg(feature = "plugin")]
    fn find_flatten_transformers(flatten: &mut Flatten) -> anyhow::Result<Vec<String>> {
        let mut props = flatten
            .values
            .iter_mut()
            .map(|val| {
                Ok(match val {
                    FlattenableValue::Object(obj) => Self::find_object_transformers(obj)?,
                    FlattenableValue::Reference(reference) => Self::map_transform(reference),
                    FlattenableValue::Plugin(gen) => Self::map_transform(gen),
                    FlattenableValue::Array(array) => Self::find_array_transformers(array)?,
                    FlattenableValue::Include(include) => {
                        let mut any = include.as_schema()?;
                        Self::find_transformers_in_any(&mut any)?
                    }
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if let Some(transform) = &flatten.transform {
            props.extend(Self::transformers_to_vec(transform, &props));
        }

        Ok(props)
    }

    #[cfg(feature = "plugin")]
    fn map_transform<T: IntoGeneratedArc>(val: &T) -> Vec<String> {
        val.get_transform()
            .map(|t| Self::transformers_to_vec(&t, &[]))
            .unwrap_or_default()
    }

    /// I am Bumblebee, I am Bumblebee, I am Bumblebee
    #[cfg(feature = "plugin")]
    fn find_transformers(any: &mut AnyValue) -> anyhow::Result<Vec<String>> {
        match any {
            AnyValue::Any(any) => Self::find_transformers_in_any(any),
            _ => Ok(vec![]),
        }
    }

    #[cfg(feature = "plugin")]
    fn find_transformers_in_any(any: &mut MaybeValidAny) -> anyhow::Result<Vec<String>> {
        Ok(match any {
            MaybeValidAny::Valid(inner) => match inner {
                Any::Object(object) => Self::find_object_transformers(object)?,
                Any::Array(array) => Self::find_array_transformers(array)?,
                Any::Flatten(flatten) => Self::find_flatten_transformers(flatten)?,
                rest => match rest {
                    Any::String(str) => str.get_transform(),
                    Any::AnyOf(any_of) => any_of.get_transform(),
                    Any::Reference(reference) => reference.get_transform(),
                    Any::Integer(integer) => GetTransform::get_transform(integer),
                    Any::Number(number) => GetTransform::get_transform(number),
                    Any::Counter(counter) => GetTransform::get_transform(counter),
                    Any::Bool(boolean) => GetTransform::get_transform(boolean),
                    Any::Plugin(plugin) => plugin.get_transform(),
                    Any::File(file) => GetTransform::get_transform(file),
                    Any::Object(_) => panic!("Object should be handled above"),
                    Any::Array(_) => panic!("Array should be handled above"),
                    Any::Flatten(_) => panic!("Flatten should be handled above"),
                    Any::Include(include) => {
                        *any = include.as_schema()?;
                        return Self::find_transformers_in_any(any);
                    }
                }
                .map(|t| Self::transformers_to_vec(&t, &[]))
                .unwrap_or_default(),
            },
            MaybeValidAny::Invalid(_) => vec![],
        })
    }

    #[cfg(feature = "plugin")]
    fn find_generators(any: &mut AnyValue) -> anyhow::Result<Vec<String>> {
        match any {
            AnyValue::Any(any) => Self::find_generators_in_any(any),
            _ => Ok(vec![]),
        }
    }

    #[cfg(feature = "plugin")]
    fn find_generators_in_any(any: &mut MaybeValidAny) -> anyhow::Result<Vec<String>> {
        Ok(match any {
            MaybeValidAny::Valid(inner) => match inner {
                Any::Plugin(gen) => vec![gen.plugin_name.clone()],
                Any::Object(obj) => obj
                    .properties
                    .iter_mut()
                    .map(|(_, val)| Self::find_generators(val))
                    .collect::<anyhow::Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect(),
                Any::Array(arr) => match arr.as_mut() {
                    Array::RandomArray(arr) => Self::find_generators(&mut arr.items)?,
                    Array::ArrayWithValues(arr) => arr
                        .values
                        .iter_mut()
                        .map(Self::find_generators)
                        .collect::<anyhow::Result<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect(),
                },
                _ => vec![],
            },
            MaybeValidAny::Invalid(_) => vec![],
        })
    }

    pub fn get<'a>(&'a self, key: &String) -> anyhow::Result<&'a Arc<dyn Plugin>> {
        self.plugins
            .get(key)
            .ok_or(anyhow!("Plugin with name '{key}' is not loaded"))
    }

    pub fn exists(&self, key: &String) -> bool {
        self.plugins.contains_key(key)
    }
}
