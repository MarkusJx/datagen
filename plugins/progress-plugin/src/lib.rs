/// A plugin that can be used to track the progress of the data generation of [`datagen_rs`].
/// The plugin will call the given callback with the current progress and
/// the total number of elements.
#[cfg(feature = "plugin")]
use datagen_rs::declare_plugin;
use datagen_rs::generate::current_schema::CurrentSchemaRef;
use datagen_rs::generate::generated_schema::GeneratedSchema;
use datagen_rs::generate::generated_schema::IntoRandom;
use datagen_rs::generate::schema_mapper::MapSchema;
use datagen_rs::plugins::plugin::Plugin;
#[cfg(feature = "plugin")]
use datagen_rs::plugins::plugin::PluginConstructor;
use datagen_rs::schema::any::Any;
use datagen_rs::schema::any_of::AnyOf;
use datagen_rs::schema::any_value::AnyValue;
use datagen_rs::schema::array::{Array, ArrayLength};
use datagen_rs::schema::object::Object;
#[cfg(not(feature = "plugin"))]
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::traits::generate::TransformTrait;
use datagen_rs::util::types::Result;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct RandomArrayLength {
    min: u32,
    max: u32,
}

impl RandomArrayLength {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

/// A plugin that can be used to track the progress of the data generation.
/// The plugin will call the given callback with the current progress and
/// the total number of elements.
pub struct ProgressPlugin<F: Fn(usize, usize)> {
    total_elements: AtomicUsize,
    progress: AtomicUsize,
    arrays: Mutex<BTreeMap<RandomArrayLength, VecDeque<u32>>>,
    callback: F,
}

impl<F: Fn(usize, usize)> Debug for ProgressPlugin<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressPlugin")
            .field("total_elements", &self.total_elements)
            .field("progress", &self.progress)
            .field("arrays", &self.arrays)
            .finish()
    }
}

#[cfg(not(feature = "plugin"))]
/// The result of the [`with_schema`](ProgressPlugin::with_schema) method.
/// Contains the schema with the progress plugin and the plugin itself.
/// The name of the plugin will be `progress`, so make sure no other plugin with that name is used.
pub struct PluginWithSchemaResult {
    /// The schema with the progress plugin.
    pub schema: Schema,
    /// The plugin map with the progress plugin.
    pub plugins: HashMap<String, Box<dyn Plugin>>,
}

impl<F: Fn(usize, usize)> ProgressPlugin<F> {
    #[cfg(not(feature = "plugin"))]
    /// Create a new progress plugin instance with the given schema and callback.
    /// The callback will be called with the current progress and the total number of elements.
    /// The schema will be modified to include the progress plugin.
    /// The name of the plugin will be `progress`, so make sure no other plugin with that name is used.
    ///
    /// # Example
    /// ```
    /// use datagen_rs::schema::any_value::AnyValue;
    /// use datagen_rs::schema::schema_definition::Schema;
    /// use datagen_rs::util::helpers::generate_random_data;
    /// use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
    ///
    /// let schema = Schema {
    ///     options: None,
    ///     value: AnyValue::String("Hello World".into())
    /// };
    ///
    /// let PluginWithSchemaResult {
    ///     schema,
    ///     plugins
    /// } = ProgressPlugin::with_schema(schema, |current, total| {
    ///     println!("{current} / {total}");
    /// }).unwrap();
    ///
    /// let generated = generate_random_data(schema, Some(plugins)).unwrap();
    /// println!("{}", generated);
    /// ```
    pub fn with_schema(mut schema: Schema, callback: F) -> Result<PluginWithSchemaResult>
    where
        F: Fn(usize, usize) + 'static,
    {
        let progress: Box<dyn Plugin> = Box::new(ProgressPlugin::new(callback));

        schema.value = AnyValue::Any(Any::Plugin(datagen_rs::schema::plugin::Plugin {
            plugin_name: "progress".into(),
            args: Some(serde_json::to_value(schema.value).map_err(|e| e.to_string())?),
            transform: None,
        }));

        Ok(PluginWithSchemaResult {
            schema,
            plugins: vec![("progress".into(), progress)].into_iter().collect(),
        })
    }

    #[cfg(not(feature = "plugin"))]
    /// Create a new progress plugin instance with the given callback.
    /// The callback will be called with the current progress and the total number of elements.
    ///
    /// A more convenient way to use this plugin is to use the
    /// [`with_schema`](ProgressPlugin::with_schema) method.
    ///
    /// # Example
    /// ```
    /// use datagen_rs::plugins::plugin::Plugin;
    /// use datagen_rs::schema::any::Any;
    /// use datagen_rs::schema::any_value::AnyValue;
    /// use datagen_rs::schema::schema_definition::Schema;
    /// use datagen_rs::util::helpers::generate_random_data;
    /// use datagen_rs_progress_plugin::ProgressPlugin;
    ///
    /// let progress: Box<dyn Plugin> = Box::new(ProgressPlugin::new(|current, total| {
    ///     println!("{current} / {total}");
    /// }));
    ///
    /// let schema = Schema {
    ///     options: None,
    ///     value: AnyValue::Any(Any::Plugin(datagen_rs::schema::plugin::Plugin {
    ///         plugin_name: "progress".into(),
    ///         args: Some(serde_json::to_value(AnyValue::String("test".into())).unwrap()),
    ///         transform: None
    ///     }))
    /// };
    ///
    /// let generated = generate_random_data(
    ///     schema,
    ///     Some(vec![("progress".into(), progress)].into_iter().collect())
    /// ).unwrap();
    /// println!("{}", generated);
    /// ```
    pub fn new(callback: F) -> Self {
        Self {
            total_elements: AtomicUsize::new(0),
            progress: AtomicUsize::new(0),
            arrays: Mutex::new(BTreeMap::new()),
            callback,
        }
    }

    fn increase_count(&self) {
        let total = self.total_elements.load(Ordering::SeqCst);
        let current = self.progress.fetch_add(1, Ordering::SeqCst) + 1;
        (self.callback)(current, total);
    }

    fn convert_any_value(
        &self,
        schema: CurrentSchemaRef,
        val: AnyValue,
    ) -> Result<Arc<GeneratedSchema>> {
        match val {
            AnyValue::Any(any) => match any {
                Any::Array(array) => self.convert_array(schema, *array),
                Any::Object(object) => self.convert_object(schema, *object),
                Any::AnyOf(any_of) => self.convert_any_of(schema, any_of),
                rest => rest.into_random(schema),
            },
            rest => rest.into_random(schema),
        }
    }

    fn get_array_length(&self, len: &ArrayLength) -> Result<u32> {
        match len {
            ArrayLength::Random { min, max } => self
                .arrays
                .lock()
                .unwrap()
                .get_mut(&RandomArrayLength::new(*min, *max))
                .ok_or("Array length not found".to_string())?
                .pop_front()
                .ok_or("Array length not found".into()),
            ArrayLength::Constant { value } => Ok(*value),
        }
    }

    fn convert_array(
        &self,
        schema: CurrentSchemaRef,
        array: Array,
    ) -> Result<Arc<GeneratedSchema>> {
        let len = self.get_array_length(&array.length)?;
        schema.map_array(
            len as _,
            array.items,
            array.transform,
            true,
            |cur, value| {
                let res = self.convert_any_value(cur.clone(), value)?;
                self.increase_count();
                Ok(res)
            },
        )
    }

    fn convert_object(
        &self,
        schema: CurrentSchemaRef,
        object: Object,
    ) -> Result<Arc<GeneratedSchema>> {
        schema.map_index_map(object.properties, object.transform, true, |cur, value| {
            let res = self.convert_any_value(cur.clone(), value)?;
            self.increase_count();
            Ok(res)
        })
    }

    fn convert_any_of(
        &self,
        schema: CurrentSchemaRef,
        mut any_of: AnyOf,
    ) -> Result<Arc<GeneratedSchema>> {
        any_of.values.shuffle(&mut rand::thread_rng());
        let mut num = any_of.num.unwrap_or(1);
        match num.cmp(&0) {
            core::cmp::Ordering::Equal => num = any_of.values.len() as i64,
            core::cmp::Ordering::Less => {
                num = rand::thread_rng().gen_range(0..any_of.values.len() as i64)
            }
            _ => {}
        }

        let values = any_of
            .values
            .drain(0..num as usize)
            .map(|value| value.into_random(schema.clone()))
            .collect::<Result<Vec<_>>>()?;

        let mut res = if values.is_empty() {
            Arc::new(GeneratedSchema::None)
        } else if values.len() == 1 {
            values[0].clone()
        } else {
            Arc::new(GeneratedSchema::Array(values))
        };

        if let Some(transform) = any_of.transform {
            res = transform.transform(schema.clone(), res)?;
        }

        Ok(schema.finalize(res))
    }

    fn map_any(&self, val: &mut AnyValue) -> usize {
        if let AnyValue::Any(any) = val {
            match any {
                Any::Array(array) => self.map_array(array.as_mut()),
                Any::Object(object) => {
                    let mut len = 1;
                    for (_, value) in &mut object.properties {
                        len += self.map_any(value);
                    }

                    len
                }
                Any::AnyOf(any_of) => {
                    any_of.values.shuffle(&mut rand::thread_rng());
                    let mut num = any_of.num.unwrap_or(1);
                    match num.cmp(&0) {
                        core::cmp::Ordering::Equal => num = -1,
                        core::cmp::Ordering::Less => {
                            num = rand::thread_rng().gen_range(0..any_of.values.len() as i64)
                        }
                        _ => {}
                    }

                    if num >= 0 {
                        any_of.values.drain(num as usize..);
                    }

                    let mut len = 1;
                    for val in &mut any_of.values {
                        len += self.map_any(val);
                    }
                    len
                }
                _ => 1,
            }
        } else {
            1
        }
    }

    fn add_array_len(&self, len: &ArrayLength) -> u32 {
        match len {
            ArrayLength::Random { min, max } => {
                let mut arrays = self.arrays.lock().unwrap();

                let entry = arrays
                    .entry(RandomArrayLength::new(*min, *max))
                    .or_insert_with(VecDeque::new);
                let mut rng = rand::thread_rng();
                let res = rng.gen_range(*min..=*max);
                entry.push_back(res);

                res
            }
            ArrayLength::Constant { value } => *value,
        }
    }

    fn map_array(&self, val: &mut Array) -> usize {
        let len = self.add_array_len(&val.length);

        let mut res = 1;
        for _ in 0..len {
            res += self.map_any(&mut val.items);
        }

        res
    }
}

impl<F: Fn(usize, usize)> Plugin for ProgressPlugin<F> {
    fn name(&self) -> String {
        "progress".into()
    }

    fn generate(&self, schema: CurrentSchemaRef, args: Value) -> Result<Arc<GeneratedSchema>> {
        let mut val: AnyValue = serde_json::from_value(args)?;

        self.total_elements
            .store(self.map_any(&mut val), Ordering::SeqCst);

        let res = self.convert_any_value(schema, val)?;
        // Increase the progress by one to account for the root element
        self.increase_count();

        Ok(res)
    }
}

#[cfg(feature = "plugin")]
impl PluginConstructor for ProgressPlugin<fn(usize, usize)> {
    fn new(_args: Value) -> Result<Self> {
        Ok(Self {
            total_elements: AtomicUsize::new(0),
            progress: AtomicUsize::new(0),
            arrays: Mutex::new(BTreeMap::new()),
            callback: |current, total| {
                println!("{current} / {total}");
            },
        })
    }
}

#[cfg(feature = "plugin")]
declare_plugin!(ProgressPlugin<fn(usize, usize)>);
