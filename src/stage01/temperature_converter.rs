use std::io;

/*
你原来的版本我先完整保留下来，并注释掉，方便你和下面更 Rust 风格的写法对比。

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
                    }
                    None => println!("Invalid number, please try again."),
                }
            }
            "f" | "fahrenheit" => {
                println!("Please input your Fahrenheit:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match parse_input(&input) {
                    Some(num) => {
                        let celsius = fahrenheit_to_celsius(num);
                        println!("{}°F is equal to {}°C", num, celsius);
                    }
                    None => println!("Invalid number, please try again."),
                }
            }
            "x" => {
                println!("Exit");
                break;
            }
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
*/

// 用枚举表示模式，而不是直接在 main 里到处匹配字符串。
// 这样语义更明确，也更符合 Rust 常见的建模方式。
enum Mode {
    Celsius,
    Fahrenheit,
    Exit,
}

fn main() {
    loop {
        print_menu();

        // 把“读取模式”和“解析模式”拆开，main 会更容易读。
        let mode = match read_mode() {
            Some(mode) => mode,
            None => {
                println!("Invalid option, please try again.");
                continue;
            }
        };

        match mode {
            Mode::Celsius => convert_celsius_to_fahrenheit(),
            Mode::Fahrenheit => convert_fahrenheit_to_celsius(),
            Mode::Exit => {
                println!("Exit");
                break;
            }
        }
    }
}

// 单独负责打印菜单。
// 把展示逻辑抽出去后，main 只保留“程序流程”。
fn print_menu() {
    println!(
        "===========================================\n\
Enter temperature converter\n\
Press (c) use celsius_to_fahrenheit\n\
Press (f) use fahrenheit_to_celsius\n\
Press (x) exit."
    );
}

// 读取用户输入，并把字符串转换成枚举。
// Option<Mode> 表示：可能识别成功，也可能失败。
fn read_mode() -> Option<Mode> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().to_lowercase().as_str() {
        "c" | "celsius" => Some(Mode::Celsius),
        "f" | "fahrenheit" => Some(Mode::Fahrenheit),
        "x" | "exit" => Some(Mode::Exit),
        _ => None,
    }
}

// 单独处理“摄氏度 -> 华氏度”流程。
// 把每个分支做成函数后，match 分支会非常干净。
fn convert_celsius_to_fahrenheit() {
    let celsius = match read_number("Please input your Celsius:") {
        Some(value) => value,
        None => {
            println!("Invalid number, please try again.");
            return;
        }
    };

    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C is equal to {fahrenheit}°F");
}

// 单独处理“华氏度 -> 摄氏度”流程。
fn convert_fahrenheit_to_celsius() {
    let fahrenheit = match read_number("Please input your Fahrenheit:") {
        Some(value) => value,
        None => {
            println!("Invalid number, please try again.");
            return;
        }
    };

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F is equal to {celsius}°C");
}

// 读取一个数字。
// 把重复的 read_line + parse 封装起来，避免两处分支写同样代码。
fn read_number(prompt: &str) -> Option<f64> {
    println!("{prompt}");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    parse_input(&input)
}

// parse 返回 Option，是 Rust 很常见的写法：
// 成功就是 Some(value)，失败就是 None。
fn parse_input(input: &str) -> Option<f64> {
    input.trim().parse().ok()
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
