fn main() {
    println!("Stage 01 / Control Flow");

    let score = 187;
    let level = if score > 100 {
        "invalid"
    } else if score >= 90 {
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

    let mut countdown = 3;
    loop {
        println!("while countdown: {countdown}");
        if countdown == 0 {
            break;
        }
        countdown -= 1;
    }

    // 半开区间 start..end
    // 闭区间 start..=end
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

    println!("试一试：");
    println!("1. 修改 score，看看 `if` 和 `match` 会如何变化。");
    println!("2. 用另一种循环形式重写其中一个循环。");
    println!("3. 为大于 100 的非法分数增加一个分支。");
}

fn letter_grade(score: u32) -> char {
    match score {
        _ if score > 100 => 'i',  // 1. 先检查是否 > 100
        90..=100 => 'A',          // 2. 然后检查 90-100
        80..=89 => 'B',           // 3. 80-89
        70..=79 => 'C',           // 4. 70-79
        66 => '6',
        60..=69 => 'D',           // 5. 60-69
        _ => 'F',                 // 6. 所有其他情况 (0-59)
    }
}