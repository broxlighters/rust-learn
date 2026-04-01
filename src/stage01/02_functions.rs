fn main() {
    println!("Stage 01 / Functions and Expressions");

    let temperature_c = 26.0;
    let temperature_f = celsius_to_fahrenheit(temperature_c);
    println!("{temperature_c:.1}C = {temperature_f:.1}F");

    // { [变量]: [填充字符][对齐方式][宽度][.精度][类型] }
    // 
    // 示例：
    // {value:0>10.3}  // 填充0，右对齐，宽度10，精度3
    // {value:<8.2}    // 左对齐，宽度8，精度2

    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{temperature_f:.1}F = {temperature_c:.1}C");

    println!("plus_one(9) = {}", plus_one(9));

    let block_value = {
        let mut base = 1;
        base += 1;
        base * 4
    };

    println!("block value = {block_value}");
    print_summary("functions", 3);

    println!("试一试：");
    println!("1. 添加一个 fahrenheit_to_celsius 函数。");
    println!("2. 修改块表达式，观察结果如何变化。");
    println!("3. 用你自己的表达方式重写 print_summary。");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit-32.0)/1.8
}

fn plus_one(value: i32) -> i32 {
    value + 1
}

fn print_summary(topic: &str, day: u32) {
    println!("Day {day}: now practicing {topic}.");
}
