use serde::Serialize;

#[derive(Serialize)]
pub struct Recipe {
    // User provided title
    title: String,
    // List of Ingredients
    ingredients: Vec<Ingredient>,
    // List of Steps
    steps: Vec<Step>,
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
