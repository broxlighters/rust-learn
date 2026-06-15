use std::env;
use std::error::Error;
use std::fs;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|message| {
        eprintln!("Argument error: {message}");
        print_help();
        std::process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Application error: {error}");
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: impl Iterator<Item = String>) -> Result<Self, String> {
        Self::build_with_ignore_case(args, env::var("IGNORE_CASE").is_ok())
    }

    fn build_with_ignore_case(
        mut args: impl Iterator<Item = String>,
        ignore_case: bool,
    ) -> Result<Self, String> {
        args.next();

        let query = args
            .next()
            .ok_or_else(|| String::from("missing query string"))?;
        let file_path = args
            .next()
            .ok_or_else(|| String::from("missing file path"))?;

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    let matches = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    if matches.is_empty() {
        println!("No matches found.");
    } else {
        for line in matches {
            println!("{line}");
        }
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn print_help() {
    println!("Usage:");
    println!("  cargo run --bin stage04_mini_grep -- <query> <file_path>");
    println!("  $env:IGNORE_CASE=1; cargo run --bin stage04_mini_grep -- <query> <file_path>");
    println!();
    println!("Examples:");
    println!("  cargo run --bin stage04_mini_grep -- frog stage04_poem.txt");
    println!("  $env:IGNORE_CASE=1; cargo run --bin stage04_mini_grep -- body stage04_poem.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust the compiler.
";

    #[test]
    fn finds_case_sensitive_matches() {
        let result = search("duct", SAMPLE);
        assert_eq!(result, vec!["safe, fast, productive."]);
    }

    #[test]
    fn finds_case_insensitive_matches() {
        let result = search_case_insensitive("pIcK", SAMPLE);
        assert_eq!(result, vec!["Pick three."]);
    }

    #[test]
    fn build_accepts_ignore_case_flag() {
        let config = Config::build_with_ignore_case(
            ["stage04_mini_grep", "rust", "sample.txt"]
                .into_iter()
                .map(String::from),
            true,
        )
        .expect("config should build");

        assert!(config.ignore_case);
    }
}
