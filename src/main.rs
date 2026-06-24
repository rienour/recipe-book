mod recipe_cli;
use std::io::Write;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use uuid::Uuid;

use crate::recipe_cli::Recipe;

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
            let mut file = match File::create_new(output_filepath) {
                Ok(file) => file,
                _ => panic!("Unknown"),
            };
            let contents = Recipe {
                title: String::from(recipe_name),
                ingredients: vec![],
                steps: vec![],
            }
            .to_string();
            file.write(contents.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
        None => {
            println!("Invalid command");
        }
    }
}
