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
        // TODO: Resolve needing unwrap?
        toml::to_string(self).unwrap()
    }
}

#[derive(Serialize)]
pub struct Ingredient {
    id: String,
    title: String,
    quantity: f64,
    unit: String,
}

#[derive(Serialize)]
pub struct Step {
    // Step in the recipe to perform the step
    ordinal_position: u16,
    // User facing description of the step
    description: String,
}
