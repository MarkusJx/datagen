use crate::generate::current_schema::CurrentSchema;
use crate::generate::generated_schema::IntoRandom;
use crate::plugins::plugin_list::PluginList;
use crate::util::helpers::{read_schema, write_json_schema};
use std::time::Instant;

mod generate;
mod plugins;
mod schema;
mod util;

fn main() {
    write_json_schema("schema.json").unwrap();

    let any = read_schema("test.json").unwrap();
    let plugins = PluginList::from_schema(&any).unwrap();
    let s = CurrentSchema::root(any.options.unwrap_or_default(), plugins);
    let start = Instant::now();
    let gen = any.value.into_random(s).unwrap();
    let duration = start.elapsed();

    let _generated = serde_json::to_string_pretty(&gen).unwrap();
    println!("{}", _generated);
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
