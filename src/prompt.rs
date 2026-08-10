use std::io::{self, Write};

// TODO: Investigate using generics or traits for increased versatility?
pub fn prompt_non_empty_string(prompt: String, same_line: bool) -> String {
    print!("{}", prompt);
    if same_line {
        print!(" ");
    }
    // TODO: Use expect?
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    while user_input.trim().len() == 0 {
        io::stdin().read_line(&mut user_input).unwrap();
    }

    user_input.trim().to_string()
}

// TODO: Investigate using generics or traits for increased versatility?
pub fn prompt_float(prompt: String) -> f64 {
    print!("{} ", prompt);
    // TODO: Use expect?
    io::stdout().flush().unwrap();

    let mut user_input = String::new();

    loop {
        io::stdin().read_line(&mut user_input).unwrap();

        let result = user_input.trim().parse::<f64>();
        match result {
            Ok(result) => {
                return result;
            }
            // TODO: Add better user feedback
            Err(_err) => {
                print!("{} ", prompt);
                // TODO: Use expect?
                io::stdout().flush().unwrap();

                println!("Enter a valid decimal value");
            }
        }
    }
}
