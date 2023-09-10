mod util;

use crate::util::cli_progress::{CliProgressRef, CliProgressTrait};
use clap::{Parser, Subcommand};
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::any::Any;
use datagen_rs::schema::any_value::AnyValue;
use datagen_rs::schema::generator::Generator;
use datagen_rs::util::helpers::{generate_random_data, read_schema, write_json_schema};
use datagen_rs::util::types::Result;
use progress_plugin::ProgressPlugin;
use std::process::exit;

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

fn generate_data(
    schema_file: String,
    out_file: Option<String>,
    progress: Box<dyn Plugin>,
    progress_bar: &CliProgressRef,
) -> Result<()> {
    let mut schema = read_schema(schema_file)?;
    schema.value = AnyValue::Any(Any::Generator(Generator {
        plugin_name: "progress".into(),
        args: Some(serde_json::to_value(schema.value)?),
        transform: None,
    }));

    let generated = generate_random_data(
        schema,
        Some(vec![("progress".into(), progress)].into_iter().collect()),
    )?;

    if let Some(out_file) = out_file {
        progress_bar.set_message("Writing results to file");
        std::fs::write(out_file, generated)?;
    } else {
        println!("{generated}");
    }

    Ok(())
}

fn main() {
    let args = CommendLineArgs::parse();

    match args.command {
        Commands::Generate {
            schema_file,
            out_file,
        } => {
            let progress_bar = CliProgressRef::default();
            let progress_bar_copy = progress_bar.clone();
            let progress: Box<dyn Plugin> = Box::new(ProgressPlugin::new(move |current, total| {
                progress_bar_copy.increase(current, total);
            }));

            let res = generate_data(schema_file, out_file, progress, &progress_bar);
            progress_bar.finish(res.is_ok());

            if let Err(err) = res {
                eprintln!("Failed to generate data: {}", err);
                exit(1);
            }
        }
        Commands::WriteJsonSchema { path } => {
            if let Err(e) = write_json_schema(path) {
                eprintln!("Failed to write json schema: {}", e);
                exit(1);
            }
        }
    }
}
