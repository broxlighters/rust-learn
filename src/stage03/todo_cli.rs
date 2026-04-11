use std::env;
use std::error::Error;
use std::io;

const DATA_FILE: &str = "todo_data.txt";

mod model {
    #[derive(Debug, Clone)]
    pub struct Task {
        pub id: u32,
        pub title: String,
        pub done: bool,
    }

    impl Task {
        pub fn new(id: u32, title: &str) -> Self {
            Self {
                id,
                title: title.to_string(),
                done: false,
            }
        }

        pub fn to_line(&self) -> String {
            let done_flag = if self.done { "1" } else { "0" };
            format!("{}\t{}\t{}", self.id, done_flag, self.title)
        }

        pub fn from_line(line: &str) -> Option<Self> {
            let mut parts = line.splitn(3, '\t');
            let id = parts.next()?.parse().ok()?;
            let done = match parts.next()? {
                "1" => true,
                "0" => false,
                _ => return None,
            };
            let title = parts.next()?.to_string();

            Some(Self { id, title, done })
        }
    }
}

mod storage {
    use crate::model::Task;
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn load_tasks(path: &str) -> io::Result<Vec<Task>> {
        if !Path::new(path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(path)?;
        let mut tasks = Vec::new();

        for (index, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let task = Task::from_line(line).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid task data on line {}", index + 1),
                )
            })?;

            tasks.push(task);
        }

        Ok(tasks)
    }

    pub fn save_tasks(path: &str, tasks: &[Task]) -> io::Result<()> {
        let lines: Vec<String> = tasks.iter().map(Task::to_line).collect();
        let content = if lines.is_empty() {
            String::new()
        } else {
            format!("{}\n", lines.join("\n"))
        };

        fs::write(path, content)
    }
}

mod app {
    use crate::model::Task;

    pub fn list_tasks(tasks: &[Task]) {
        if tasks.is_empty() {
            println!("No tasks yet.");
            return;
        }

        for task in tasks {
            let status = if task.done { "[x]" } else { "[ ]" };
            println!("#{} {} {}", task.id, status, task.title);
        }
    }

    pub fn add_task(tasks: &mut Vec<Task>, title: &str) {
        let id = next_id(tasks);
        tasks.push(Task::new(id, title));
        println!("Added task #{}: {}", id, title);
    }

    pub fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), String> {
        let task = tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("Task #{} not found.", id))?;

        task.done = true;
        println!("Marked task #{} as done.", id);
        Ok(())
    }

    pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
        let position = tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or_else(|| format!("Task #{} not found.", id))?;

        let removed = tasks.remove(position);
        println!("Deleted task #{}: {}", removed.id, removed.title);
        Ok(())
    }

    fn next_id(tasks: &[Task]) -> u32 {
        tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
    }
}

use app::{add_task, delete_task, list_tasks, mark_done};
use storage::{load_tasks, save_tasks};

fn main() {
    println!("Stage 03 / Todo CLI");

    if let Err(error) = run() {
        eprintln!("Error: {error}");
        print_help();
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks(DATA_FILE)?;

    match args.get(1).map(String::as_str) {
        Some("list") => {
            list_tasks(&tasks);
        }
        Some("add") => {
            let title = args.get(2..).map(|parts| parts.join(" ")).unwrap_or_default();

            if title.trim().is_empty() {
                return Err(invalid_input("Please provide a task title."));
            }

            add_task(&mut tasks, &title);
            save_tasks(DATA_FILE, &tasks)?;
        }
        Some("done") => {
            let id = parse_task_id(args.get(2))?;
            mark_done(&mut tasks, id).map_err(|message| invalid_input(&message))?;
            save_tasks(DATA_FILE, &tasks)?;
        }
        Some("delete") => {
            let id = parse_task_id(args.get(2))?;
            delete_task(&mut tasks, id).map_err(|message| invalid_input(&message))?;
            save_tasks(DATA_FILE, &tasks)?;
        }
        Some("help") | None => {
            print_help();
        }
        Some(command) => {
            return Err(invalid_input(&format!("Unknown command: {command}")));
        }
    }

    Ok(())
}

fn parse_task_id(raw: Option<&String>) -> Result<u32, Box<dyn Error>> {
    let input = raw.ok_or_else(|| invalid_input("Please provide a task id."))?;
    Ok(input.parse()?)
}

fn invalid_input(message: &str) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::InvalidInput, message.to_string()))
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --bin stage03_todo_cli -- list");
    println!("  cargo run --bin stage03_todo_cli -- add \"learn Vec\"");
    println!("  cargo run --bin stage03_todo_cli -- done 1");
    println!("  cargo run --bin stage03_todo_cli -- delete 1");
}
