use crate::util::helpers::{generate_random_data, read_schema, write_json_schema};

mod generate;
mod plugins;
mod schema;
mod util;

fn main() {
    write_json_schema("schema.json").unwrap();

    let any = read_schema("test.json").unwrap();
    let generated = generate_random_data(any, None).unwrap();

    println!("{}", generated);
    //println!("Time elapsed in expensive_function() is: {:?}", duration);
}
