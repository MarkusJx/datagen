#[cfg(feature = "plugin")]
use datagen_rs::declare_plugin;
use datagen_rs::generate::current_schema::CurrentSchema;
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
use datagen_rs::util::types::Result;
use rand::prelude::SliceRandom;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct ProgressPlugin {
    total_elements: AtomicUsize,
    progress: AtomicUsize,
    callback: fn(max: usize, current: usize),
}

impl ProgressPlugin {
    #[cfg(not(feature = "plugin"))]
    pub fn new(callback: fn(max: usize, current: usize)) -> Self {
        Self {
            total_elements: AtomicUsize::new(0),
            progress: AtomicUsize::new(0),
            callback,
        }
    }

    fn convert_any_value(
        &self,
        schema: Arc<CurrentSchema>,
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

    fn convert_array(
        &self,
        schema: Arc<CurrentSchema>,
        array: Array,
    ) -> Result<Arc<GeneratedSchema>> {
        schema.map_array(
            array.length.get_length() as _,
            array.items,
            array.transform,
            true,
            |cur, value| {
                let progress = self.progress.fetch_add(1, Ordering::Relaxed);
                let total = self.total_elements.load(Ordering::Relaxed);
                (self.callback)(total, progress);
                self.convert_any_value(cur.clone(), value)
            },
        )
    }

    fn convert_object(
        &self,
        schema: Arc<CurrentSchema>,
        object: Object,
    ) -> Result<Arc<GeneratedSchema>> {
        schema.map_index_map(object.properties, object.transform, true, |cur, value| {
            self.convert_any_value(cur.clone(), value)
        })
    }

    fn convert_any_of(
        &self,
        schema: Arc<CurrentSchema>,
        mut any_of: AnyOf,
    ) -> Result<Arc<GeneratedSchema>> {
        any_of.values.shuffle(&mut rand::thread_rng());
        let values = any_of
            .values
            .drain(0..any_of.num.unwrap_or(1) as usize)
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

    pub fn map_any(&self, val: &mut AnyValue) {
        if let AnyValue::Any(any) = val {
            match any {
                Any::Array(array) => self.map_array(array.as_mut()),
                Any::Object(object) => {
                    for (_, value) in &mut object.properties {
                        self.map_any(value);
                    }
                }
                Any::AnyOf(any_of) => {
                    for val in &mut any_of.values {
                        self.map_any(val);
                    }
                }
                _ => {}
            }
        }
    }

    fn map_array(&self, val: &mut Array) {
        let len = val.length.get_length();
        val.length = ArrayLength::Constant { value: len };

        self.total_elements.fetch_add(len as _, Ordering::Relaxed);
        self.map_any(&mut val.items);
    }
}

impl Plugin for ProgressPlugin {
    fn name(&self) -> &'static str {
        "progress"
    }

    fn generate(&self, schema: Arc<CurrentSchema>, args: Value) -> Result<Arc<GeneratedSchema>> {
        let mut val: AnyValue = serde_json::from_value(args)?;

        self.convert_any_value(schema, val)
    }
}

#[cfg(feature = "plugin")]
impl PluginConstructor for ProgressPlugin {
    fn new(args: Box<Value>) -> Result<Self> {
        todo!()
    }
}

#[cfg(feature = "plugin")]
declare_plugin!(ProgressPlugin);
