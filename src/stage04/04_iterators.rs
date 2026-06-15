#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
    priority: u8,
}

fn main() {
    println!("Stage 04 / Iterators");
    println!("This example shows iterator chains, closures, and collect.\n");

    let tasks = vec![
        Task {
            title: String::from("read the Rust book chapter"),
            done: true,
            priority: 1,
        },
        Task {
            title: String::from("finish mini-grep"),
            done: false,
            priority: 3,
        },
        Task {
            title: String::from("write iterator notes"),
            done: false,
            priority: 2,
        },
    ];

    let pending_titles: Vec<&str> = tasks
        .iter()
        .filter(|task| !task.done)
        .map(|task| task.title.as_str())
        .collect();

    println!("pending titles = {:?}", pending_titles);

    let high_priority: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.priority >= 2)
        .collect();

    println!("high priority tasks = {:?}", high_priority);

    let doubled = vec![1, 2, 3, 4]
        .into_iter()
        .map(|number| number * 2)
        .collect::<Vec<i32>>();

    println!("doubled numbers = {:?}", doubled);

    println!("\nTry this:");
    println!("1. Add another filter condition.");
    println!("2. Replace collect with count or any.");
    println!("3. Rewrite one iterator chain with a for loop and compare.");
}
