mod recipe_cli;
use std::io::{Read, Write};
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::recipe_cli::{Ingredient, Recipe, Step};

#[derive(Subcommand)]
enum Commands {
    /// Command to create a recipe given the file path and name
    CreateExample {
        /// Filepath the resulting recipe file will be written to
        #[arg(short, long)]
        output_file: PathBuf,
        /// Name of the recipe to create
        #[arg(short, long)]
        recipe_name: String,
    },
    // Verify a recipe is a valid file
    Verify {
        /// File to read the recipe from
        #[arg(short, long)]
        file: PathBuf,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::CreateExample {
            output_file,
            recipe_name,
        }) => {
            let mut file = match File::create_new(output_file) {
                Ok(file) => file,
                Err(err) => panic!("{}", err),
            };
            let contents = Recipe {
                title: String::from(recipe_name),
                ingredients: vec![
                    Ingredient {
                        id: String::from("example"),
                        title: String::from("Example Ingredient"),
                        quantity: 1.0,
                        unit: String::from("gram"),
                    },
                    Ingredient {
                        id: String::from("example"),
                        title: String::from("Example Ingredient"),
                        quantity: 1.0,
                        unit: String::from("gram"),
                    },
                ],
                steps: vec![Step {
                    ordinal_position: 1,
                    description: String::from("Example step detailing information"),
                }],
            }
            .to_string();
            file.write(contents.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
        Some(Commands::Verify { file }) => {
            // TODO: Clean-up implementation
            let file = file.clone().into_os_string().into_string().unwrap();
            let mut open_file = File::open(&file).unwrap();
            let mut file_contents = String::new();

            open_file.read_to_string(&mut file_contents).unwrap();
            let read_content = toml::from_str::<Recipe>(file_contents.as_mut_str()).unwrap();

            println!("{}", file_contents);
            println!("{:?}", read_content);
        }
        None => {
            println!("Invalid command");
        }
    }
}
