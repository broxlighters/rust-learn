use std::io;

/*
你原来的版本我先完整保留下来，并注释掉，方便你和下面更 Rust 风格的写法对比。

fn main() {
    let left = match read_number("left") {
        Some(number) => number,
        None => {
            println!("Invalid left number");
            return;
        }
    };

    let operation = match read_operation() {
        Some(operation) => operation,
        None => {
            println!("Invalid operation");
            return;
        }
    };

    let right = match read_number("right") {
        Some(number) => number,
        None => {
            println!("Invalid right number");
            return;
        }
    };

    match calculate(left, right, &operation) {
        Ok(result) => println!("{} {} {} = {}", left, operation, right, result),
        Err(error) => println!("{}", error),
    }
}

fn read_number(number_type: &str) -> Option<f64> {
    println!("Enter a {} number:", number_type);
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

fn read_operation() -> Option<String> {
    println!("Enter an operation (+, -, *, /):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    match input.trim() {
        "+" | "-" | "*" | "/" => Some(input.trim().to_string()),
        _ => None,
    }
}

fn calculate(left: f64, right: f64, operation: &str) -> Result<f64, &str> {
    match operation {
        "+" => Ok(add(left, right)),
        "-" => Ok(subtract(left, right)),
        "*" => Ok(multiply(left, right)),
        "/" => divide(left, right),
        _ => Err("Invalid operation"),
    }
}

fn add(left: f64, right: f64) -> f64 {
    left + right
}

fn subtract(left: f64, right: f64) -> f64 {
    left - right
}

fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

fn divide(left: f64, right: f64) -> Result<f64,  &str> {
    if right == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(left / right)
    }
}
*/

fn main() {
    let Some(left) = read_number("left") else {
        println!("Invalid left number");
        return;
    };

    let Some(operation) = read_operation() else {
        println!("Invalid operation");
        return;
    };

    let Some(right) = read_number("right") else {
        println!("Invalid right number");
        return;
    };

    match calculate(left, right, operation) {
        Ok(result) => println!("{left} {operation} {right} = {result}"),
        Err(error) => println!("{error}"),
    }
}

// 读取一个数字。
// 返回 Option，表示“可能成功解析，也可能失败”。
fn read_number(number_type: &str) -> Option<f64> {
    println!("Enter a {number_type} number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

// 读取运算符。
// 这里只接受 + - * / 四种符号，其他输入直接返回 None。
fn read_operation() -> Option<&'static str> {
    println!("Enter an operation (+, -, *, /):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    match input.trim() {
        "+" => Some("+"),
        "-" => Some("-"),
        "*" => Some("*"),
        "/" => Some("/"),
        _ => None,
    }
}

// 核心计算逻辑放在一个函数里。
// 这样更贴近练习文档里提到的“封装成 calculate 函数”。
fn calculate(left: f64, right: f64, operation: &str) -> Result<f64, &'static str> {
    match operation {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" if right == 0.0 => Err("Cannot divide by zero"),
        "/" => Ok(left / right),
        _ => Err("Invalid operation"),
    }
}
