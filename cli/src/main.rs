use clap::{Parser, Subcommand};
use colored::Colorize;
use datagen_rs::plugins::plugin::Plugin;
use datagen_rs::schema::any::Any;
use datagen_rs::schema::any_value::AnyValue;
use datagen_rs::schema::generator::Generator;
use datagen_rs::util::helpers::{generate_random_data, read_schema, write_json_schema};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use progress_plugin::ProgressPlugin;
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

#[derive(Default)]
struct CliProgress {
    pb: Option<ProgressBar>,
}

impl CliProgress {
    fn increase(&mut self, current: usize, total: usize) {
        if self.pb.is_none() {
            println!(
                "Generating {} records",
                format!("~{}", total.to_formatted_string(&Locale::en)).bright_cyan()
            );

            let len = total.to_string().len() + 1;
            let pb = ProgressBar::new(total as _);
            pb.enable_steady_tick(Duration::from_millis(80));
            pb.set_style(
                ProgressStyle::with_template(
                    &format!("{{spinner:.green}} {{msg}} [{{elapsed_precise}}] |{{wide_bar:.cyan/blue}}| {{cur:>{len}}}/{{total:{len}}} ({{per_sec}})"),
                )
                .unwrap()
                .with_key("cur", |state: &ProgressState, w: &mut dyn Write| write!(w, "{}", state.pos().to_formatted_string(&Locale::en)).unwrap())
                .with_key("total", |state: &ProgressState, w: &mut dyn Write| write!(w, "{}", state.len().unwrap_or_default().to_formatted_string(&Locale::en)).unwrap())
                .with_key("per_sec", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}/s", state.per_sec()).unwrap())
                .progress_chars("#>-")
                .tick_strings(&[
                            "⠋",
                            "⠙",
                            "⠹",
                            "⠸",
                            "⠼",
                            "⠴",
                            "⠦",
                            "⠧",
                            "⠇",
                            "⠏",
                            "✔"
                ]),
            );
            pb.set_message("Generating records");

            self.pb.replace(pb);
        }

        self.pb.as_ref().unwrap().set_position(current as _);
    }

    fn set_message(&self, msg: &'static str) {
        if let Some(pb) = self.pb.as_ref() {
            pb.set_message(msg);
        }
    }

    fn finish(&self) {
        if let Some(pb) = self.pb.as_ref() {
            pb.finish_with_message("Done");
            println!(
                "Success - Generated {} records in {}",
                pb.position().to_formatted_string(&Locale::en).bright_blue(),
                format!("{:.1?}", pb.elapsed()).bright_blue()
            );
        }
    }
}

fn main() {
    let args = CommendLineArgs::parse();

    match args.command {
        Commands::Generate {
            schema_file,
            out_file,
        } => {
            let mut schema = read_schema(schema_file).unwrap();
            let progress_bar = Arc::new(Mutex::new(CliProgress::default()));
            let progress = progress_bar.clone();
            let progress: Box<dyn Plugin> = Box::new(ProgressPlugin::new(move |current, total| {
                let mut progress_bar = progress.lock().unwrap();
                progress_bar.increase(current, total);
            }));

            schema.value = AnyValue::Any(Any::Generator(Generator {
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
                progress_bar
                    .lock()
                    .unwrap()
                    .set_message("Writing results to file");
                std::fs::write(out_file, generated).unwrap();
            } else {
                println!("{generated}");
            }
            progress_bar.lock().unwrap().finish();
        }
        Commands::WriteJsonSchema { path } => {
            write_json_schema(path).unwrap();
        }
    }
}
