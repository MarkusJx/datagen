use clap::{Parser, Subcommand};
use datagen_rs::util::helpers::{generate_random_data, read_schema, write_json_schema};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CommendLineArgs {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Write the JSON schema to a file
    WriteJsonSchema {
        path: String
    },
    /// Generate random data from a schema
    Generate {
        /// The path to the schema file to use
        schema_file: String,
        /// An optional path to write the generated data to.
        /// If not specified, the data will be written to stdout.
        out_file: Option<String>,
    }
}

fn main() {
    let args = CommendLineArgs::parse();

    match args.command {
        Commands::Generate { schema_file, out_file} => {
            let schema = read_schema(schema_file).unwrap();
            let generated = generate_random_data(schema).unwrap();

            if let Some(out_file) = out_file {
                std::fs::write(out_file, generated).unwrap();
            } else {
                println!("{generated}");
            }
        },
        Commands::WriteJsonSchema { path } => {
            write_json_schema(path).unwrap();
        }
    }
}
