use std::io::Write;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use serde::Serialize;
use toml::Table;
use uuid::Uuid;

#[derive(Subcommand)]
enum Commands {
    /// Command to create a recipe given the file path and name
    Create {
        /// Filepath the resulting recipe file will be written to
        #[arg(short, long)]
        output_filepath: PathBuf,
        /// Name of the recipe to create
        #[arg(short, long)]
        recipe_name: String,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    println!("{}", Uuid::new_v4());

    let args = Cli::parse();

    match &args.command {
        Some(Commands::Create {
            output_filepath,
            recipe_name,
        }) => {
            // TODO: Resolve using unwrapping
            let mut file = match File::create_new(output_filepath) {
                Ok(file) => file,
                _ => panic!("Unknown"),
            };
            let contents = toml::to_string(&Recipe {
                test: String::from("string"),
                test2: vec![String::from("test"), String::from("hi")],
            })
            .unwrap();
            file.write(contents.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
            // let value = "foo = 'bar'\n[keys]\ntest = [1, { test = \"hi\"\n, test2 = 2\n }]"
            //     .parse::<Table>()
            //     .unwrap();
            // println!("{:?}", value);
        }
        None => {
            println!("Invalid command");
        }
    }
}
