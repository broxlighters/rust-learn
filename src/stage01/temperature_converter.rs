use std::io;

fn main() {
    loop {
        println!("\
        ===========================================\n \
        Enter temperature converter\n \
        Press (c) use celsius_to_fahrenheit\n \
        Press (f) use fahrenheit_to_celsius\n \
        Press (x) exit.\n ");

        let mut mode = String::new();
        io::stdin().read_line(&mut mode).expect("Failed to read line");
        let mut input = String::new();
        let mode = mode.trim().to_lowercase();
        match mode.as_str() {
            "c" | "celsius" => {
                println!("Please input your Celsius:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match parse_input(&input) {
                    Some(num) => {
                        let fahrenheit = celsius_to_fahrenheit(num);
                        println!("{}°C is equal to {}°F", num, fahrenheit);
                    },
                    None => println!("Invalid number, please try again."),
                }
            },
            "f" | "fahrenheit" => {
                println!("Please input your Fahrenheit:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match parse_input(&input) {
                    Some(num) => {
                        let celsius = fahrenheit_to_celsius(num);
                        println!("{}°F is equal to {}°C", num, celsius);
                    },
                    None => println!("Invalid number, please try again."),
                }
            },
            "x" => {
                println!("Exit");
                break;
            },
            _ => {
                println!("Invalid option, please try again.")
            }
        }
    }
}

fn parse_input(input: &str) -> Option<f64> {
    input.trim().parse().ok()
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}