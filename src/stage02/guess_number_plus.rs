use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Stage 02 / Guess Number Plus");

    let mut total_rounds = 0;
    let mut win_rounds = 0;

    loop {
        total_rounds += 1;

        if play_round() {
            win_rounds += 1;
        }

        println!("score: {win_rounds}/{total_rounds}");

        if !ask_continue() {
            println!("Bye!");
            break;
        }
    }
}

fn play_round() -> bool {
    let secret = rand::rng().random_range(1..=100);
    println!("Guess the number between 1 and 100.");

    loop {
        let input = read_input("Enter your guess (or q to quit this round):");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Round ended.");
            return false;
        }

        let guess = match parse_guess(&input) {
            Some(number) => number,
            None => {
                println!("Invalid number, please try again.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return true;
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn parse_guess(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

fn ask_continue() -> bool {
    let answer = read_input("Play again? (y/n)");
    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}
