use serde::Serialize;

#[derive(Serialize)]
pub struct Recipe {
    // User provided title
    pub title: String,
    // List of Ingredients
    pub ingredients: Vec<Ingredient>,
    // List of Steps
    pub steps: Vec<Step>,
}

impl ToString for Recipe {
    fn to_string(&self) -> String {
        // TODO: Resolve needing unwrap
        toml::to_string(self).unwrap()
    }
}

#[derive(Serialize)]
pub struct Ingredient {
    pub id: String,
    pub title: String,
    pub quantity: f64,
    pub unit: String,
}

#[derive(Serialize)]
pub struct Step {
    // Step in the recipe to perform the step
    pub ordinal_position: u16,
    // User facing description of the step
    pub description: String,
}
