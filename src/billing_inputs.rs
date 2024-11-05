use std::io;

pub fn get_string() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter data again");
    }
    let input: String = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn get_amount() -> Option<f64> {
    println!("Enter an amount:");
    loop {
        let input: String = match get_string() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None
        }
        let parsed_to_f64: Result<f64, _> = input.parse();
        match parsed_to_f64 {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Incorrect input, please enter a number"),
        }
    }
}