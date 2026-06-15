fn main() {
    println!("Stage 04 / Traits");
    println!("This example shows trait definitions, impl Trait, and trait bounds.\n");

    let article = Article {
        headline: String::from("Rust traits make shared behavior explicit"),
        author: String::from("brox"),
        content: String::from("Traits describe capabilities that many types can share."),
    };

    let note = StatusNote {
        username: String::from("rustacean"),
        content: String::from("Iterator chains are easier to read after practice."),
    };

    notify(&article);
    notify(&note);

    let summary = returns_summarizable();
    println!("returned summary: {}", summary.summarize());

    println!("\nTry this:");
    println!("1. Add another type that implements Summary.");
    println!("2. Change notify to take a generic parameter.");
    println!("3. Add a default method to the trait.");
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    author: String,
    content: String,
}

struct StatusNote {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!(
            "{} by {} ({})",
            self.headline,
            self.author,
            preview(&self.content)
        )
    }
}

impl Summary for StatusNote {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("notification: {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    StatusNote {
        username: String::from("library"),
        content: String::from("impl Trait can hide the concrete return type."),
    }
}

fn preview(content: &str) -> String {
    const LIMIT: usize = 24;

    if content.chars().count() <= LIMIT {
        return content.to_string();
    }

    let snippet: String = content.chars().take(LIMIT).collect();
    format!("{snippet}...")
}
