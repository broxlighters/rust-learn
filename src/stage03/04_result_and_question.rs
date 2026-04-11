use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Stage 03 / Result and ?");
    println!("本节目标：理解 Result 返回值、? 错误传播，以及基础文件读取。\n");

    match parse_task_id("42") {
        Ok(id) => println!("解析成功，任务 id = {id}"),
        Err(error) => println!("解析失败：{error}"),
    }

    match parse_task_id("abc") {
        Ok(id) => println!("解析成功，任务 id = {id}"),
        Err(error) => println!("解析失败：{error}"),
    }

    write_demo_file()?;
    let content = read_demo_file()?;
    println!("读取到的文件内容：{content}");

    println!("\n本节结论：");
    println!("1. Result 用来表示可能成功也可能失败的操作。");
    println!("2. ? 可以把错误直接往上返回，少写重复的 match。");
    println!("3. 文件读写失败通常应该返回错误，而不是直接 panic。\n");

    println!("试一试：");
    println!("1. 把 parse_task_id 的输入改成别的字符串。");
    println!("2. 修改写入的文件内容，再运行看看输出。");
    println!("3. 故意读取一个不存在的文件，观察错误信息。");

    Ok(())
}

fn parse_task_id(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse()
}

fn write_demo_file() -> io::Result<()> {
    fs::write("stage03_demo.txt", "learn Result and question mark")
}

fn read_demo_file() -> io::Result<String> {
    fs::read_to_string("stage03_demo.txt")
}
