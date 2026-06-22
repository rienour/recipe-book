use std::io::Write;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
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
            // TODO: Resolve using unwrap
            let mut file = File::create_new(output_filepath).unwrap();
            file.write(recipe_name.t);
            file.write(b"\n");
        }
        None => {
            println!("Invalid command");
        }
    }
}

// fn main() {
//     let mut file = File::create_new("foo.txt").unwrap();
//     file.write(b"Test 2").unwrap();
// }
