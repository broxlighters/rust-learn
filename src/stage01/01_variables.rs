const MAX_POINTS: u32 = 100;

fn main() {
    println!("Stage 01 / Variables and Types");

    let topic = "variables";
    let topic = topic.to_uppercase();

    let mut score = 60;
    {
        let score = 90;
        println!("score shadowing = {score}")
    }
    score += 15;

    let progress = score as f64 / MAX_POINTS as f64;
    let guess: u32 = "42".parse().expect("hard-coded number should parse");

    let course = ("Rust", 1, true, false);
    let lucky_numbers = [3, 9, 21, 42];

    println!("topic = {topic}");
    println!("score = {score}");
    println!("progress = {:.0}%", progress * 100.0);
    println!("guess = {guess}");
    println!(
        "course = ({}, {}, {}, {})",
        course.0, course.1, course.2, course.3
    );
    println!("first lucky number = {}", lucky_numbers[0]);
    println!("sum = {}", add(lucky_numbers[0], lucky_numbers[1]));

    println!("试一试：");
    println!("1. 修改 score，比较 `mut` 和 shadowing 的区别。");
    println!("2. 再添加一个元组，并打印它的字段。");
    println!("3. 替换数组中的值，然后重新计算总和。");
}

fn add(left: u32, right: u32) -> u32 {
    left + right
}
