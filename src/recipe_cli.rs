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

    pub fn set_ingredients(&mut self, ingredients: Vec<Ingredient>) -> &Self {
        self.ingredients = ingredients;
        self
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> &Self {
        self.ingredients.push(ingredient);
        self
    }

    pub fn set_steps(&mut self, steps: Vec<Step>) -> &Self {
        self.steps = steps;
        self
    }

    pub fn add_step(&mut self, step: Step) -> &Self {
        self.steps.push(step);
        self
    }

    pub fn reset(&mut self) -> &Self {
        self.title = String::default();
        self.ingredients = Vec::default();
        self.steps = Vec::default();
        self
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

pub struct IngredientBuilder {
    id: String,
    title: String,
    quantity: f64,
    unit: String,
}

impl IngredientBuilder {
    pub fn new() -> Self {
        Self {
            id: String::default(),
            title: String::default(),
            quantity: f64::default(),
            unit: String::default(),
        }
    }

    pub fn set_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_quantity(mut self, quantity: f64) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn set_unit(mut self, unit: String) -> Self {
        self.unit = unit;
        self
    }

    pub fn reset(mut self) -> Self {
        self.id = String::default();
        self.title = String::default();
        self.quantity = f64::default();
        self.unit = String::default();
        self
    }

    pub fn build(self) -> Ingredient {
        Ingredient {
            id: self.id,
            title: self.title,
            quantity: self.quantity,
            unit: self.unit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    // Step in the recipe to perform the step
    pub ordinal_position: u16,
    // User facing description of the step
    pub description: String,
}

pub struct StepBuilder {
    ordinal_position: u16,
    description: String,
}

impl StepBuilder {
    pub fn new() -> Self {
        StepBuilder {
            ordinal_position: u16::default(),
            description: String::default(),
        }
    }

    pub fn set_ordinal_position(mut self, ordinal_position: u16) -> Self {
        self.ordinal_position = ordinal_position;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn reset(mut self) -> Self {
        self.ordinal_position = u16::default();
        self.description = String::default();
        self
    }

    pub fn build(self) -> Step {
        Step {
            ordinal_position: self.ordinal_position,
            description: self.description,
        }
    }
}
