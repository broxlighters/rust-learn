fn main() {
    println!("Stage 02 / Borrowing");
    println!("本节目标：理解借用就是“临时访问”，不是“拿走所有权”。\n");

    let book = String::from("The Rust Programming Language");

    // 这里把 &book 传进去，函数只能读，不能拿走 book。
    print_length(&book);

    // 因为前面只是借用，所以这里仍然可以继续使用原值。
    println!("book is still available = {book}");

    let note = "borrowing lets you read without taking ownership";
    print_message(note);

    println!("\n错误示例（已注释）：");
    println!("看文件里的注释代码，理解为什么传值后原变量会失效。\n");

    /*
    let article = String::from("Rust Book");
    take_string(article);
    println!("{article}");
    // 错误原因：article 已经被传给 take_string，
    // 所有权被拿走，所以后面不能再使用。
    */

    println!("本节结论：");
    println!("1. 借用不会转移所有权。\n2. 只读函数通常优先考虑 &str。\n3. 借用结束后，原值仍然可以继续使用。");

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
