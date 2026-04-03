fn main() {
    println!("Stage 02 / Borrowing");

    let book = String::from("The Rust Programming Language");
    print_length(&book);
    println!("book is still available = {book}");

    let note = "borrowing lets you read without taking ownership";
    print_message(note);

    println!("试一试：");
    println!("1. 把 &book 改成 book，看看为什么后面不能再使用它。");
    println!("2. 再写一个只读函数，参数类型改成 &str。");
    println!("3. 思考什么时候函数参数应该写 String，什么时候写 &str。");
}

fn print_length(text: &String) {
    println!("length = {}", text.len());
}

fn print_message(message: &str) {
    println!("message = {message}");
}
