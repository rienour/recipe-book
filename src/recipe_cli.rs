// TODO: Roll builders into associated structs?
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    // TODO: Move from String to a SemVer type?
    // Schema version the recipe was written with
    schema_version: String,
    // TODO: Update to use Uuid type with custom serialization?
    // Unique identifier for the recipe
    id: String,
    // User provided title
    pub title: String,
    // List of Ingredients
    pub ingredients: Vec<Ingredient>,
    // List of Steps
    pub steps: Vec<Step>,
}

impl Recipe {
    pub fn new(title: String, ingredients: Vec<Ingredient>, steps: Vec<Step>) -> Self {
        Self {
            schema_version: env!("CARGO_PKG_VERSION").to_string(),
            id: Uuid::new_v4().to_string(),
            title,
            ingredients,
            steps,
        }
    }
}

// TODO: Should this be Display instead?
impl ToString for Recipe {
    fn to_string(&self) -> String {
        // TODO: Resolve needing unwrap
        toml::to_string(self).unwrap()
    }
}

pub struct RecipeBuilder {
    // User provided title
    title: String,
    // List of Ingredients
    ingredients: Vec<Ingredient>,
    // List of Steps
    steps: Vec<Step>,
}

impl RecipeBuilder {
    pub fn new() -> Self {
        Self {
            title: String::default(),
            ingredients: Vec::default(),
            steps: Vec::default(),
        }
    }

    pub fn set_title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> &Self {
        self.ingredients.push(ingredient);
        self
    }

    pub fn add_step(&mut self, step: Step) -> &Self {
        self.steps.push(step);
        self
    }

    pub fn step_count(&self) -> u16 {
        // TODO: Review typing to determine if u16 is the proper type for this field
        self.steps.len().try_into().unwrap()
    }

    pub fn build(self) -> Recipe {
        Recipe::new(self.title, self.ingredients, self.steps)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub id: String,
    pub title: String,
    // TODO: Update to an Option type with custom serialization?
    pub quantity: f64,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    // Step in the recipe to perform the step
    pub ordinal_position: u16,
    // User facing description of the step
    pub description: String,
}
