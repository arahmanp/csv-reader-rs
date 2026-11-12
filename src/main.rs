use std::{env, fs, path::Path, process::ExitCode};
use colored::Colorize;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}", "Need one argument!".red().bold());
        return ExitCode::FAILURE;
    }

    let path = &args[1];

    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    if extension != "csv" {
        eprintln!("{}", "Invalid file type!".red().bold());
        return ExitCode::FAILURE;
    }

    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("{}", "Failed to read file!".red().bold());
            return ExitCode::FAILURE;
        }
    };

    let content = content.trim();

    if content.is_empty() {
        eprintln!("{}", "FIle is empty!");
        return ExitCode::FAILURE;
    }

    let data: Vec<&str> = content.lines().collect();

    for line in data {
        println!("{line}");
    }

    ExitCode::SUCCESS
}
