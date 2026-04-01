fn main() {
    println!("Stage 01 / Functions and Expressions");

    let temperature_c = 26.0;
    let temperature_f = celsius_to_fahrenheit(temperature_c);

    println!("{temperature_c:.1}C = {temperature_f:.1}F");
    println!("plus_one(9) = {}", plus_one(9));

    let block_value = {
        let base = 3;
        base * 4
    };

    println!("block value = {block_value}");
    print_summary("functions", 3);

    println!("Try:");
    println!("1. Add a fahrenheit_to_celsius function.");
    println!("2. Change the block expression and observe the result.");
    println!("3. Rewrite print_summary with your own wording.");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn plus_one(value: i32) -> i32 {
    value + 1
}

fn print_summary(topic: &str, day: u32) {
    println!("Day {day}: now practicing {topic}.");
}
