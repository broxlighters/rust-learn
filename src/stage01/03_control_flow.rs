fn main() {
    println!("Stage 01 / Control Flow");

    let score = 87;
    let level = if score >= 90 {
        "excellent"
    } else if score >= 60 {
        "pass"
    } else {
        "retry"
    };

    println!("score = {score}, level = {level}");
    println!("letter grade = {}", letter_grade(score));

    let mut countdown = 3;
    while countdown > 0 {
        println!("while countdown: {countdown}");
        countdown -= 1;
    }

    for day in 1..=3 {
        println!("for day: {day}");
    }

    let mut attempts = 0;
    loop {
        attempts += 1;
        println!("loop attempt: {attempts}");

        if attempts == 2 {
            break;
        }
    }

    println!("Try:");
    println!("1. Change score and see how `if` and `match` react.");
    println!("2. Rewrite one loop using another loop form.");
    println!("3. Add a branch for invalid scores greater than 100.");
}

fn letter_grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}
