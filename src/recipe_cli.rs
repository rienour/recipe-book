use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    // Schema version the recipe was written with
    schema_version: String,
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
            title,
            ingredients,
            steps,
        }
    }
}

impl ToString for Recipe {
    fn to_string(&self) -> String {
        // TODO: Resolve needing unwrap
        toml::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub id: String,
    pub title: String,
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
