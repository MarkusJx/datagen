mod util;

use crate::util::cli_progress::{CliProgressRef, CliProgressTrait};
use clap::{Parser, Subcommand};
use datagen_rs::util::helpers::{generate_random_data, read_schema, write_json_schema};
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
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
    progress_bar: &CliProgressRef,
) -> anyhow::Result<Option<String>> {
    let progress_bar_copy = progress_bar.clone();
    let PluginWithSchemaResult { schema, plugins } =
        ProgressPlugin::with_schema(read_schema(schema_file)?, move |current, total| {
            progress_bar_copy.increase(current, total);
        })?;

    let generated = generate_random_data(schema, Some(plugins))?;

    if let Some(out_file) = out_file {
        progress_bar.set_message("Writing results to file");
        std::fs::write(out_file, generated)?;

        Ok(None)
    } else {
        Ok(Some(generated))
    }
}

fn main() {
    let args = CommendLineArgs::parse();

    match args.command {
        Commands::Generate {
            schema_file,
            out_file,
        } => {
            let progress_bar = CliProgressRef::default();

            let res = generate_data(schema_file, out_file, &progress_bar);
            progress_bar.finish(res.is_ok());

            match res {
                Err(err) => {
                    eprintln!("Failed to generate data: {}", err);
                    exit(1);
                }
                Ok(Some(generated)) => {
                    println!("{generated}");
                }
                Ok(None) => {}
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
