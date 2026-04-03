fn main() {
    println!("Stage 02 / References and Mutable Borrowing");

    let mut message = String::from("hello");
    append_world(&mut message);
    println!("message = {message}");

    // 同一时刻只能有一个可变借用。
    // 也不能在可变借用存在时再创建不可变借用。
    let length = message.len();
    println!("length = {length}");

    println!("试一试：");
    println!("1. 在 append_world 调用前后添加新的借用，看看哪些写法会报错。");
    println!("2. 再写一个函数，把字符串末尾追加感叹号。");
    println!("3. 思考为什么 Rust 要限制可变借用数量。");
}

fn append_world(text: &mut String) {
    text.push_str(", world");
}
