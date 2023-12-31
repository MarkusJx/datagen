mod util;

use crate::util::cli_progress::{CliProgressRef, CliProgressTrait};
use clap::{Parser, Subcommand};
use datagen_rs::generate::current_schema::CurrentSchema;
use datagen_rs::generate::generated_schema::IntoRandom;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::plugins::plugin_list::PluginList;
use datagen_rs::schema::schema_definition::Schema;
use datagen_rs::util::helpers::{read_schema, write_json_schema};
#[cfg(feature = "node")]
use datagen_rs_node_runner::runner::node_runner::NodeRunner;
use datagen_rs_progress_plugin::{PluginWithSchemaResult, ProgressPlugin};
use std::collections::HashMap;
use std::process::exit;
use std::sync::Arc;

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

fn generate_random_data(
    schema: Schema,
    additional_plugins: Option<HashMap<String, Box<dyn Plugin>>>,
    progress_bar: CliProgressRef,
) -> anyhow::Result<(String, Arc<PluginList>)> {
    let plugins = PluginList::from_schema(&schema, additional_plugins)?;
    let options = Arc::new(schema.options.unwrap_or_default());
    let root = CurrentSchema::root(options.clone(), plugins.clone());
    let generated = schema.value.into_random(root)?;

    progress_bar.set_message("Serializing data");

    Ok((
        options
            .serializer
            .as_ref()
            .unwrap_or_default()
            .serialize_generated_with_progress(
                generated,
                Some(plugins.clone()),
                &|current, total| {
                    progress_bar.increase(current, total);
                },
            )?,
        plugins,
    ))
}

fn generate_data(
    schema_file: String,
    out_file: Option<String>,
    progress_bar: &CliProgressRef,
) -> anyhow::Result<Option<String>> {
    let progress_bar_copy = progress_bar.clone();
    #[cfg_attr(not(feature = "node"), allow(unused_mut))]
    let PluginWithSchemaResult {
        schema,
        mut plugins,
    } = ProgressPlugin::with_schema(read_schema(schema_file)?, move |current, total| {
        progress_bar_copy.increase(current, total);
    })?;

    #[cfg(feature = "node")]
    let (_runner, node_plugins) = NodeRunner::init(&schema)?;
    #[cfg(feature = "node")]
    plugins.extend(node_plugins);
    let (generated, _plugins) = generate_random_data(schema, Some(plugins), progress_bar.clone())?;

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
                    eprintln!("{:?}", err.context("Failed to generate data"));
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
