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

    let content: Vec<&str> = content.lines().collect();

    let header: Vec<&str> = content[0].split(',').collect();

    let mut body: Vec<Vec<&str>> = Vec::with_capacity(content.len() - 1);

    for i in 1..(content.len()) {
        let record: Vec<&str> = content[i].split(',').collect();

        if record.len() != header.len() {
            eprintln!("{}", "Record size is not matching with header size!".red().bold());
            return ExitCode::FAILURE;
        }

        body.push(record);
    }

    println!("{:?}", header);

    for record in body {
        println!("{:?}", record);
    }

    ExitCode::SUCCESS
}
