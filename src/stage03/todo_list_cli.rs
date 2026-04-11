mod model {
    pub struct Task {
        name: String,
        completed: bool,
    }

    impl Task {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                completed: false,
            }
        }
        pub fn get_name(&self) -> &str {
            &self.name
        }
        pub fn is_completed(&self) -> bool {
            self.completed
        }
        pub fn to_string(&self) -> String {
            /*
            let status = if self.completed { "[x]" } else { "[ ]" };
            status.to_string() + " " + &self.name
            */
            let status = if self.is_completed() { "[x]" } else { "[ ]" };
            format!("{status} {}", self.get_name())
        }
        pub fn mark_completed(&mut self) {
            self.completed = true;
        }
        pub fn unmark_completed(&mut self) {
            self.completed = false;
        }
        pub fn print_task(&self) {
            /*
            let status = if self.completed { "[x]" } else { "[ ]" };
            println!("{status} {}", self.name);
            */
            println!("{}", self.to_string());
        }
    }
}

mod cli {
    use std::io;

    pub enum Command {
        Add(String),
        Complete(usize),
        UnComplete(usize),
        List,
        Help,
        Exit
    }

    /*
    pub fn parse_command() -> Option<Command> {
    */
    pub fn parse_command() -> Result<Command, String> {
        let mut input = String::new();
        /*
        io::stdin().read_line(&mut input).ok()?;
        */
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .map_err(|error| format!("读取输入失败: {error}"))?;

        if bytes_read == 0 {
            return Ok(Command::Exit);
        }

        let input = input.trim();
        /*
        let parts: Vec<&str> = input.split(' ').collect();
        println!("Parsed command: {:?}", parts[0]);
        match parts[0] {
        */
        let parts: Vec<&str> = input.split_whitespace().collect();
        let Some(command_name) = parts.first().copied() else {
            return Err("Please enter a command.".to_string());
        };

        match command_name {
            "a" | "add" =>  {
                /*
                if parts.len() < 2 {
                    println!("请提供任务内容");
                    None
                } else {
                    let task = parts[1..].join(" ");
                    Some(Command::Add(task))
                }
                */
                let task = parts.get(1..).map(|parts| parts.join(" ")).unwrap_or_default();

                if task.trim().is_empty() {
                    Err("Please provide a task title.".to_string())
                } else {
                    Ok(Command::Add(task))
                }
            },
            /*
            "c" | "complete" => Some(Command::Complete(parts[1].parse().unwrap())),
            "u" | "un_complete" => Some(Command::UnComplete(parts[1].parse().unwrap())),
            "ls" | "list" => Some(Command::List),
            "h" | "help" => Some(Command::Help),
            "e" | "exit" => Some(Command::Exit),
            _ => None,
            */
            "c" | "complete" => Ok(Command::Complete(parse_task_index(parts.get(1).copied())?)),
            "u" | "un_complete" | "uncomplete" => {
                Ok(Command::UnComplete(parse_task_index(parts.get(1).copied())?))
            }
            "ls" | "list" => Ok(Command::List),
            "h" | "help" => Ok(Command::Help),
            "e" | "exit" => Ok(Command::Exit),
            _ => Err(format!("Unknown command: {command_name}")),
        }
    }

    fn parse_task_index(raw: Option<&str>) -> Result<usize, String> {
        let raw = raw.ok_or_else(|| "Please provide a task number.".to_string())?;
        let index: usize = raw
            .parse()
            .map_err(|_| "Task number must be a positive integer.".to_string())?;

        if index == 0 {
            return Err("Task number starts from 1.".to_string());
        }

        Ok(index - 1)
    }

    pub fn print_help() {
        /*
        println!("Commands:");
        println!("  add <task> - Add a new task");
        println!("  complete <number> - Complete a task");
        println!("  list - List all tasks");
        */
        println!("Commands:");
        println!("  add <task> - Add a new task");
        println!("  complete <number> - Complete a task by list number");
        println!("  un_complete <number> - Mark a task as not completed");
        println!("  list - List all tasks");
        println!("  help - Show help information");
        println!("  exit - Exit the program");
    }

    pub fn print_welcome() {
        println!("Welcome to the Todo List!");
    }
}

/* use std::io; */
use model::Task;
use cli::{Command, parse_command, print_help, print_welcome};
fn main () {
    let mut tasks = vec![];
    loop {
        print_welcome();
        print_help();

        /*
        let Some(command) = parse_command() else {
            println!("Invalid command, please try again!");
            return;
        };
        */
        let command = match parse_command() {
            Ok(command) => command,
            Err(message) => {
                println!("{message}");
                continue;
            }
        };

        match command {
            Command::Add(name) => {
                tasks.push(Task::new(&name));
                println!("Added task: {}",  name);
            },
            Command::Complete(index) => {
                /*
                if index < tasks.len() {
                    tasks[index].mark_completed();
                    println!("Completed task: {}", tasks[index].get_name());
                } else {
                    println!("Invalid task index");
                }
                */
                if let Some(task) = tasks.get_mut(index) {
                    task.mark_completed();
                    println!("Completed task: {}", task.get_name());
                } else {
                    println!("Invalid task index");
                }
            },
            Command::UnComplete(index) => {
                /*
                if index < tasks.len() {
                    tasks[index].unmark_completed();
                    println!("Uncompleted task: {}", tasks[index].get_name());
                } else {
                    println!("Invalid task index");
                }
                */
                if let Some(task) = tasks.get_mut(index) {
                    task.unmark_completed();
                    println!("Uncompleted task: {}", task.get_name());
                } else {
                    println!("Invalid task index");
                }
            },
            Command::Help => {
                print_help();
            },
            Command::List => {
                println!("Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    /*
                    println!("{}: {}", i + 1, task.to_string())
                    */
                    print!("{}: ", i + 1);
                    task.print_task();
                }
            },
            Command::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }
}
