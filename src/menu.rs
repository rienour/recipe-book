use std::{fmt::Display, io};

pub struct Menu<T: Display> {
    prompt_text: String,
    options: Vec<T>,
}

impl<T: Display> Menu<T> {
    pub fn new(prompt_text: String, options: Vec<T>) -> Self {
        Menu {
            prompt_text,
            options,
        }
    }

    pub fn prompt(&self) -> &T {
        loop {
            println!("{}", self.prompt_text);
            for (index, option) in self.options.iter().enumerate() {
                println!("{}: {option}", index + 1);
            }
            println!("Enter option: ");

            let mut user_input = String::new();
            // TODO: Handle errors better for user feedback
            io::stdin().read_line(&mut user_input).unwrap();

            match user_input.trim().parse::<usize>() {
                Ok(value) => {
                    if value > 0 && value <= self.options.len() {
                        return &self.options[value - 1];
                    } else {
                        println!("Invalid option \"{}\". Please try again.", value);
                    }
                }
                Err(e) => {
                    println!("Option \"{:?}\" is unknown. Please try again.", e);
                }
            }
        }
    }
}
