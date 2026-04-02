use std::num::ParseIntError;

fn main() {
    println!("Stage 01 / Option and Result");

    let known_student = find_student(2);
    let missing_student = find_student(99);
    // let error_student = find_student(101);

    println!("student #2 = {}", match known_student {
        Some(name) => name,
        None => "unknown",
    });
    println!("student #99 = {}", missing_student.unwrap_or("unknown"));
    // println!("student #0 = {}", error_student.unwrap_or("unknown"));

    print_score("95");
    print_score("abc");

    println!("试一试：");
    println!("1. 在 find_student 中再添加一个学生。");
    println!("2. 用 match 替代 unwrap_or。");
    println!("3. 拒绝大于 100 的分数。");
}

fn find_student(id: u32) -> Option<&'static str> {
    match id {
        _ if id > 100 => panic!("score is too high"),
        1 => Some("Alice"),
        2 => Some("Bob"),
        3 => Some("Carol"),
        4 => Some("Dave"),
        5 => Some("Edward"),
        _ => Some("unknown"),
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
