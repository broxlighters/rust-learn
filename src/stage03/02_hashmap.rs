use std::collections::HashMap;

fn main() {
    println!("Stage 03 / HashMap");
    println!("本节目标：理解 HashMap 的插入、读取、更新，以及用它做简单统计。\n");

    let mut map = HashMap::new();
    map.insert("Rust", 95);
    map.get("Rust");

    let mut scores = HashMap::new();
    scores.insert(String::from("Rust"), 95);
    scores.insert(String::from("Cargo"), 88);
    scores.insert(String::from("mod"), 88);

    println!("初始成绩表 = {:?}", scores);

    let rust_score = scores.get("Rust");
    println!("Rust 分数 = {:?}", rust_score);

    scores.insert(String::from("Rust"), 100);
    println!("更新后成绩表 = {:?}", scores);

    let words = ["rust", "cargo", "rust", "vec", "rust", "cargo"];
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for (index,word) in words.iter().enumerate() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("词频统计 = {:?}", counts);

    println!("\n本节结论：");
    println!("1. HashMap 适合按 key 查找数据。");
    println!("2. insert 可以新增，也可以覆盖旧值。");
    println!("3. entry(...).or_insert(...) 很适合做计数统计。");

    println!("\n试一试：");
    println!("1. 再加一个课程分数，看看表里怎么变化。");
    println!("2. 统计一组新的单词出现次数。");
    println!("3. 试着读取一个不存在的 key，看看 get 返回什么。");
}
