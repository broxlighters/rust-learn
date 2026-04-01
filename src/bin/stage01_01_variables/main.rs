const MAX_POINTS: u32 = 100;

fn main() {
    println!("Stage 01 / Variables and Types");

    let topic = "variables";
    let topic = topic.to_uppercase();

    let mut score = 60;
    score += 15;

    let progress = score as f64 / MAX_POINTS as f64;
    let guess: u32 = "42".parse().expect("hard-coded number should parse");

    let course = ("Rust", 1, true);
    let lucky_numbers = [3, 7, 21, 42];

    println!("topic = {topic}");
    println!("score = {score}");
    println!("progress = {:.0}%", progress * 100.0);
    println!("guess = {guess}");
    println!(
        "course = ({}, {}, {})",
        course.0, course.1, course.2
    );
    println!("first lucky number = {}", lucky_numbers[0]);
    println!("sum = {}", add(lucky_numbers[0], lucky_numbers[1]));

    println!("Try:");
    println!("1. Change score and compare `mut` with shadowing.");
    println!("2. Add one more tuple and print its fields.");
    println!("3. Replace array values and recalculate the sum.");
}

fn add(left: u32, right: u32) -> u32 {
    left + right
}
