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
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct ProgressPlugin<F: Fn(usize, usize)> {
    total_elements: AtomicUsize,
    progress: AtomicUsize,
    callback: F,
}

impl<F: Fn(usize, usize)> Debug for ProgressPlugin<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressPlugin")
            .field("total_elements", &self.total_elements)
            .field("progress", &self.progress)
            .finish()
    }
}

impl<F: Fn(usize, usize)> ProgressPlugin<F> {
    #[cfg(not(feature = "plugin"))]
    pub fn new(callback: F) -> Self {
        Self {
            total_elements: AtomicUsize::new(0),
            progress: AtomicUsize::new(0),
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
        let len = array.length.get_length();
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

    pub fn map_any(val: &mut AnyValue) -> usize {
        if let AnyValue::Any(any) = val {
            match any {
                Any::Array(array) => Self::map_array(array.as_mut()),
                Any::Object(object) => {
                    let mut len = 0;
                    for (_, value) in &mut object.properties {
                        len += Self::map_any(value);
                    }

                    len
                }
                Any::AnyOf(any_of) => {
                    any_of.values.shuffle(&mut rand::thread_rng());
                    any_of.values.drain(any_of.num.unwrap_or(1)..);
                    let mut len = 0;

                    for val in &mut any_of.values {
                        len += Self::map_any(val);
                    }
                    len
                }
                _ => 0,
            }
        } else {
            0
        }
    }

    fn map_array(val: &mut Array) -> usize {
        let len = val.length.get_length();
        val.length = ArrayLength::Constant { value: len };

        Self::map_any(&mut val.items) * len as usize + len as usize
    }
}

impl<F: Fn(usize, usize)> Plugin for ProgressPlugin<F> {
    fn name(&self) -> &'static str {
        "progress"
    }

    fn generate(&self, schema: Arc<CurrentSchema>, args: Value) -> Result<Arc<GeneratedSchema>> {
        let mut val: AnyValue = serde_json::from_value(args)?;

        self.total_elements
            .store(Self::map_any(&mut val), Ordering::SeqCst);
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
