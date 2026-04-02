#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    finished: bool,
}

impl Book {
    fn get_status(&self) -> &'static str {
        if self.finished {
            "finished"
        } else {
            "reading"
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
    Blue
}

fn main() {
    println!("Stage 01 / Structs and Enums");

    let rust_book = Book {
        title: String::from("The Rust Book"),
        author: String::from("The Rust author"),
        pages: 560,
        finished: false,
    };

    let rust_book_2 = Book {
        title: String::from("The Rust Book"),
        author: String::from("The Rust author"),
        pages: 560,
        finished: true,
    };

    // {:?} 是调试格式化（debug formatting）的占位符，用于打印实现了 Debug trait 的类型
    println!("book = {:?}", rust_book);
    println!("title = {}", rust_book.title);
    println!("author = {}", rust_book.author);
    println!("status = {}", book_status(&rust_book));
    println!("pages = {}", rust_book.pages);

    println!("book_1's status = {:?}", rust_book.get_status());
    println!("book_2's status = {:?}", rust_book_2.get_status());

    for light in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green, TrafficLight::Blue] {
        println!("light = {:?}, action = {}", light, describe_light(light));
    }

    println!("试一试：");
    println!("1. 给 Book 添加一个 author 字段。");
    println!("2. 再添加一个枚举变体，并在 match 中处理它。");
    println!("3. 创建第二本 Book，并比较它们的状态。");
}

fn book_status(book: &Book) -> &'static str {
    if book.finished {
        "finished"
    } else {
        "reading"
    }
}

fn describe_light(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "stop",
        TrafficLight::Yellow => "slow down",
        TrafficLight::Green => "go",
        TrafficLight::Blue => "unknown",
    }
}
