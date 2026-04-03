fn main() {
    println!("Stage 02 / Slice");
    println!("本节目标：理解 slice 是对现有数据的一段借用。\n");

    let phrase = String::from("hello rust world");

    let first = first_word(&phrase);
    let hello = &phrase[0..5];
    let rust = &phrase[6..10];
    let all = &phrase[..];

    println!("first word = {first}");
    println!("hello = {hello}");
    println!("rust = {rust}");
    println!("all = {all}");

    println!("\n错误示例（已注释）：");
    println!("看文件里的注释代码，理解为什么在保留 slice 后不能随意修改原字符串。\n");

    /*
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{word}");
    // 错误原因：word 仍然借用了 s 的一部分，
    // 这时不能再把 s 清空。
    */

    println!("本节结论：");
    println!("1. slice 不拥有数据。\n2. &str 本质上就是字符串 slice。\n3. 返回 slice 往往比返回新的 String 更轻量。");

    println!("试一试：");
    println!("1. 改变字符串内容，观察切片范围如何变化。");
    println!("2. 让 first_word 返回第二个单词。");
    println!("3. 思考为什么 slice 不拥有数据。");
}

fn first_word(text: &str) -> &str {
    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &text[..index];
        }
    }

    text
}
