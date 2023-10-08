use datagen_rs::plugins::plugin::Plugin;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::Ref;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type DropRefsTsfn = ThreadsafeFunction<()>;
pub type PluginMap = HashMap<String, Box<dyn Plugin>>;
pub type PluginMapResult = anyhow::Result<(PluginMap, DropRefsTsfn)>;
pub type RefArc = Arc<Mutex<Ref<()>>>;
