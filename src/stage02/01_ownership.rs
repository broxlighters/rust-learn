fn main() {
    println!("Stage 02 / Ownership");

    let title = String::from("Rust ownership");
    let moved_title = title;
    println!("moved title = {moved_title}");

    let cloned_title = moved_title.clone();
    println!("cloned title = {cloned_title}");
    print_and_take(cloned_title);

    let score = 100;
    let copied_score = score;
    println!("score = {score}, copied_score = {copied_score}");

    println!("试一试：");
    println!("1. 注释掉 clone，看看哪一行会报 move 错误。");
    println!("2. 把 i32 换成 String，观察 Copy 和 Move 的区别。");
    println!("3. 修改 print_and_take 的参数，思考怎样避免所有权转移。");
}

fn print_and_take(text: String) {
    println!("taken text = {text}");
}
