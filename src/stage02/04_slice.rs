fn main() {
    println!("Stage 02 / Slice");

    let phrase = String::from("hello rust world");
    let first = first_word(&phrase);
    let hello = &phrase[0..5];
    let rust = &phrase[6..10];
    let all = &phrase[..];

    println!("first word = {first}");
    println!("hello = {hello}");
    println!("rust = {rust}");
    println!("all = {all}");

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
