use std::path::PathBuf;

use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Subcommand)]
enum Commands {
    /// Command to create a recipe given the file path and name
    Create {
        /// Filepath the resulting recipe file will be written to
        #[arg(short, long, default_value = ".")]
        output_filepath: PathBuf,
        /// Name of the recipe to create
        #[arg(short, long)]
        recipe_name: PathBuf,
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
            println!("outdir: {:?}, name: {:?}", output_filepath, recipe_name);
        }
        None => {
            println!("Invalid command");
        }
    }
}
