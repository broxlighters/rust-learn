use std::num::ParseIntError;

fn main() {
    println!("Stage 01 / Option and Result");

    let known_student = find_student(2);
    let missing_student = find_student(99);

    println!("student #2 = {}", known_student.unwrap_or("unknown"));
    println!("student #99 = {}", missing_student.unwrap_or("unknown"));

    print_score("95");
    print_score("abc");

    println!("Try:");
    println!("1. Add one more student to find_student.");
    println!("2. Use match instead of unwrap_or.");
    println!("3. Reject scores greater than 100.");
}

fn find_student(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Alice"),
        2 => Some("Bob"),
        3 => Some("Carol"),
        _ => None,
    }
}

fn parse_score(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse::<u32>()
}

fn print_score(input: &str) {
    match parse_score(input) {
        Ok(score) => println!("parsed score = {score}"),
        Err(error) => println!("failed to parse `{input}`: {error}"),
    }
}
