fn main() {
    println!("Stage 02 / References and Mutable Borrowing");
    println!("本节目标：理解可变借用为什么要受到严格限制。\n");

    let mut message = String::from("hello");
    println!("before = {message}");

    append_world(&mut message);
    println!("after = {message}");

    let length = message.len();
    println!("length = {length}");

    println!("\n错误示例（已注释）：");
    println!("看文件里的注释代码，理解为什么不能同时保留多个冲突借用。\n");

    /*
    let mut text = String::from("hello");
    let r1 = &text;
    let r2 = &mut text;
    println!("{} {}", r1, r2);
    // 错误原因：不可变借用 r1 还在使用时，
    // 又创建了可变借用 r2。
    */

    println!("本节结论：");
    println!("1. &mut T 允许修改原值。\n2. 同一时刻只能有一个可变借用。\n3. 可变借用和不可变借用不能同时存在。");

    println!("试一试：");
    println!("1. 在 append_world 调用前后添加新的借用，看看哪些写法会报错。");
    println!("2. 再写一个函数，把字符串末尾追加感叹号。");
    println!("3. 思考为什么 Rust 要限制可变借用数量。");
}

fn append_world(text: &mut String) {
    text.push_str(", world");
}
