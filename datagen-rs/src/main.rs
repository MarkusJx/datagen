use crate::generate::current_schema::CurrentSchema;
use crate::generate::generated_schema::IntoRandom;
use crate::plugins::plugin_list::PluginList;
use crate::util::helpers::{generate_random_data, read_schema, write_json_schema};
use std::time::Instant;

mod generate;
mod plugins;
mod schema;
mod util;

fn main() {
    write_json_schema("schema.json").unwrap();

    let any = read_schema("test.json").unwrap();
    let generated = generate_random_data(any).unwrap();

    println!("{}", generated);
    //println!("Time elapsed in expensive_function() is: {:?}", duration);
}
