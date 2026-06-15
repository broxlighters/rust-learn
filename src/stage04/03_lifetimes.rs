fn main() {
    println!("Stage 04 / Lifetimes");
    println!("This example shows why returned references need explicit relationships.\n");

    let left = String::from("short");
    let right = String::from("a much longer string");
    let result = longest(left.as_str(), right.as_str());

    println!("longest string = {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("novel should contain a sentence");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("excerpt part = {}", excerpt.part);
    println!("announcement result = {}", excerpt.announce_and_return_part("reading note"));

    println!("\nTry this:");
    println!("1. Pass different strings to longest.");
    println!("2. Change ImportantExcerpt to borrow another slice.");
    println!("3. Explain what reference lives long enough for the return value.");
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention: {announcement}");
        self.part
    }
}
