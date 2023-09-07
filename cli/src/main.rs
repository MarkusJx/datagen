use clap::{Parser, Subcommand};
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::any::Any;
use datagen_rs::schema::any_value::AnyValue;
use datagen_rs::schema::generator::Generator;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::{generate_random_data, read_schema, write_json_schema};
use progress_plugin::{ProgressPlugin};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CommendLineArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write the JSON schema to a file
    WriteJsonSchema { path: String },
    /// Generate random data from a schema
    Generate {
        /// The path to the schema file to use
        schema_file: String,
        /// An optional path to write the generated data to.
        /// If not specified, the data will be written to stdout.
        out_file: Option<String>,
    },
}

fn main() {
    println!("Hello, world!");
    let args = CommendLineArgs::parse();

    match args.command {
        Commands::Generate {
            schema_file,
            out_file,
        } => {
            let mut schema = read_schema(schema_file).unwrap();
            let progress = ProgressPlugin::new(|max, current| {
                println!("{} / {}", current, max);
            });

            progress.map_any(&mut schema.value);
            let progress: Box<dyn Plugin> = Box::new(progress);
            schema.value = AnyValue::Any(Any::Generator( Generator {
                plugin_name: "progress".into(),
                args: Some(serde_json::to_value(schema.value).unwrap()),
                transform: None,
            }));

            let generated = generate_random_data(
                schema,
                Some(vec![("progress".into(), progress)].into_iter().collect()),
            )
            .unwrap();

            if let Some(out_file) = out_file {
                std::fs::write(out_file, generated).unwrap();
            } else {
                //println!("{generated}");
            }
        }
        Commands::WriteJsonSchema { path } => {
            write_json_schema(path).unwrap();
        }
    }
}
