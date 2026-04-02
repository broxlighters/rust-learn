use std::io;

/*
你原来的版本我先完整保留下来，并注释掉，方便你和下面更 Rust 风格的写法对比。

fn main() {
    println!("Enter a number:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n = match parse_input(&n) {
        Some(n) => n,
        None => {
            println!("Invalid number.");
            return;
        }
    };

    println!("Fibonacci sequence:");
    let mut memo = vec![None; n + 1];

    for i in 0..=n {
        println!("{}", fib(i, &mut memo));
    }
}

fn parse_input(input: &str) -> Option<usize> {
    input.trim().parse().ok()
}

fn fib(n: usize, memo: &mut Vec<Option<u64>>) -> u64 {
    if let Some(value) = memo[n] {
        return value;
    }

    let value = if n < 2 {
        n as u64
    } else {
        fib(n - 1, memo) + fib(n - 2, memo)
    };

    memo[n] = Some(value);
    value
}
*/

fn main() {
    let Some(n) = read_number() else {
        println!("Invalid number.");
        return;
    };

    println!("Fibonacci sequence:");
    let mut memo = vec![None; n + 1];

    for i in 0..=n {
        println!("{}", fib(i, &mut memo));
    }
}

// 把读取输入的细节封装起来，main 只保留主流程。
fn read_number() -> Option<usize> {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

// 递归 + 缓存（memoization）。
// 用切片而不是 Vec 类型，参数更通用，也更符合 Rust 风格。
fn fib(n: usize, memo: &mut [Option<u64>]) -> u64 {
    if let Some(value) = memo[n] {
        return value;
    }

    let value = match n {
        0 | 1 => n as u64,
        _ => fib(n - 1, memo) + fib(n - 2, memo),
    };

    memo[n] = Some(value);
    value
}
