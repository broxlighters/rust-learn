fn main() {
    println!("Stage 02 / Ownership");
    println!("本节目标：理解 move、clone、Copy 的区别。\n");

    // String 在堆上分配数据，不是 Copy 类型。
    // 所以把它赋值给另一个变量时，会发生所有权转移（move）。
    let title = String::from("Rust ownership");
    let moved_title = title;
    println!("moved title = {moved_title}");

    // 如果后面还想继续使用同样的数据，可以显式 clone 一份。
    let cloned_title = moved_title.clone();
    // 所有权转移后，moved_title 就不再可用了
    // println!("cloned title = {title}");
    println!("cloned title = {moved_title}");
    println!("cloned title = {cloned_title}");

    // 这里传入的是借用，所以 cloned_title 不会被拿走。
    print_and_borrow(&cloned_title);
    println!("cloned_title is still available = {cloned_title}");

    let s = String::from("hello");  // s 进入作用域
    print_and_take(s);             // s 的值移动到函数里 ...
    // println!("{}",s);               // ... 所以到这里不再有效


    let score = 100;
    let copied_score = score;
    println!("score = {score}, copied_score = {copied_score}");

    println!("\n错误示例（已注释）：");
    println!("看文件里的注释代码，理解为什么 move 之后原变量不能继续使用。\n");
    /*
    let name = String::from("Rust");
    let moved_name = name;
    println!("{}", name);
    // 错误原因：name 的所有权已经 move 给 moved_name，
    // 所以后面不能再继续使用 name。
    */

    println!("本节结论：");
    println!("1. String 默认会 move。\n2. clone 会复制数据。\n3. i32 这类 Copy 类型赋值后原值仍然可用。");

    println!("试一试：");
    println!("1. 注释掉 clone，看看哪一行会报 move 错误。");
    println!("2. 把 i32 换成 String，观察 Copy 和 Move 的区别。");
    println!("3. 修改 print_and_take 的参数，思考怎样避免所有权转移。");
}

fn print_and_borrow(text: &str) {
    println!("taken text = {text}");
}
fn print_and_take(some_string: String) { // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放