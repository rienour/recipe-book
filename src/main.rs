mod menu;
mod recipe_cli;
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Read, Write},
};
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::menu::Menu;
use crate::recipe_cli::{Ingredient, Recipe, RecipeBuilder, Step};

#[derive(Subcommand)]
enum Commands {
    /// Command to create a recipe template given the file path and name
    CreateTemplate {
        /// Filepath the resulting recipe file will be written to
        #[arg(short, long)]
        output_file: PathBuf,
        /// Name of the recipe to create
        #[arg(short, long)]
        recipe_name: String,
    },
    /// Command to interactively create a recipe
    Create {},
    // Verify a recipe is a valid file
    Verify {
        /// File to read the recipe from
        #[arg(short, long)]
        file: PathBuf,
    },
}

// TODO: Move somewhere more appropriate in the Crate?
#[derive(PartialEq)]
enum CreateOption {
    UpdateName,
    AddStep,
    AddIngredient,
    SaveAndExit,
    Exit,
}

impl CreateOption {
    pub fn description(&self) -> String {
        match self {
            CreateOption::UpdateName => "Update Name".to_string(),
            CreateOption::AddStep => "Add Step".to_string(),
            CreateOption::AddIngredient => "Add Ingredient".to_string(),
            CreateOption::SaveAndExit => "Save".to_string(),
            CreateOption::Exit => "Exit".to_string(),
        }
    }

    pub fn into_vec() -> Vec<CreateOption> {
        return vec![
            CreateOption::UpdateName,
            CreateOption::AddStep,
            CreateOption::AddIngredient,
            CreateOption::SaveAndExit,
            CreateOption::Exit,
        ];
    }
}

impl Display for CreateOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        // TODO: Investigate better ways to dispatch commands
        Some(Commands::CreateTemplate {
            output_file,
            recipe_name,
        }) => {
            let mut file = match File::create_new(output_file) {
                Ok(file) => file,
                Err(err) => panic!("{}", err),
            };
            let contents = Recipe::new(
                String::from(recipe_name),
                vec![Ingredient {
                    id: String::from("example"),
                    title: String::from("Example Ingredient"),
                    quantity: 1.0,
                    unit: String::from("gram"),
                }],
                vec![Step {
                    ordinal_position: 1,
                    description: String::from("Example step detailing information"),
                }],
            )
            .to_string();
            file.write(contents.as_bytes()).unwrap();
            file.write(b"\n").unwrap();
        }
        Some(Commands::Create {}) => {
            let mut recipe_builder = RecipeBuilder::new();
            let mut user_input = String::new();
            let menu = Menu::new("Select an option".to_string(), CreateOption::into_vec());
            loop {
                let response = menu.prompt();
                match *response {
                    CreateOption::UpdateName => {
                        let user_input =
                            prompt_non_empty_string("Enter recipe name:".to_string(), true);

                        recipe_builder.set_title(user_input);
                    }
                    CreateOption::Exit => {
                        println!("{}", response);
                        break;
                    }
                    CreateOption::SaveAndExit => {
                        // TODO: Update to handle validating the string is a valid file path?
                        let user_input =
                            prompt_non_empty_string("Enter filepath:".to_string(), true);

                        let mut file = match File::create_new(user_input) {
                            Ok(file) => file,
                            // TODO: Update to gracefully handle and provide more user feedback
                            Err(err) => panic!("{}", err),
                        };
                        let contents = recipe_builder.build().to_string();
                        file.write(contents.as_bytes()).unwrap();
                        file.write(b"\n").unwrap();
                        break;
                    }
                    _ => {
                        println!("{}: Not yet implemented", response);
                    }
                }

                user_input.clear();
            }
            println!("Create called");
        }
        Some(Commands::Verify { file }) => {
            // TODO: Clean-up implementation
            let file = file.clone().into_os_string().into_string().unwrap();
            let mut open_file = File::open(&file).unwrap();
            let mut file_contents = String::new();

            open_file.read_to_string(&mut file_contents).unwrap();
            let read_content = toml::from_str::<Recipe>(file_contents.as_mut_str()).unwrap();

            println!("{}", file_contents);
            println!("{:#?}", read_content);
        }
        None => {
            println!("Invalid command");
        }
    }
}

// TODO: Investigate using generics or traits for increased versatility?
fn prompt_non_empty_string(prompt: String, same_line: bool) -> String {
    print!("{}", prompt);
    if same_line {
        print!(" ");
    } else {
        // TODO: Use expect?
        io::stdout().flush().unwrap();
    }

    let mut user_input = String::new();
    while user_input.trim().len() == 0 {
        io::stdin().read_line(&mut user_input).unwrap();
    }

    user_input
}
