#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
    finished: bool,
}

#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    println!("Stage 01 / Structs and Enums");

    let rust_book = Book {
        title: String::from("The Rust Book"),
        pages: 560,
        finished: false,
    };

    println!("book = {:?}", rust_book);
    println!("title = {}", rust_book.title);
    println!("status = {}", book_status(&rust_book));
    println!("pages = {}", rust_book.pages);

    for light in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        println!("light = {:?}, action = {}", light, describe_light(light));
    }

    println!("Try:");
    println!("1. Add an author field to Book.");
    println!("2. Add another enum variant and handle it in match.");
    println!("3. Create a second Book and compare their status.");
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
    }
}
