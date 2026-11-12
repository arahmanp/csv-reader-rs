use std::{cmp, env, fs, path::Path, process::ExitCode};
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
    let n_column = header.len();

    let mut body: Vec<Vec<&str>> = Vec::with_capacity(content.len() - 1);

    let mut table_width = 0;
    let mut column_width = vec![0; n_column];

    for i in 1..(content.len()) {
        let record: Vec<&str> = content[i].split(',').collect();

        if record.len() != n_column {
            eprintln!("{}", "Record size is not matching with header size!".red().bold());
            return ExitCode::FAILURE;
        }

        for j in 0..n_column {
            column_width[j] = cmp::max(column_width[j], record[j].len() + 2);
        }

        body.push(record);
    }

    for w in column_width.iter() {
        table_width += w;
    }

    table_width += 2 * n_column - 1;

    print!("+");

    for _i in 0..table_width {
        print!("-");
    }

    print!("+\n");

    print!("| ");

    for i in 0..n_column {
        let len = column_width[i];
        print!("{:<len$}| ", header[i]);
    }

    print!("\n+");

    for _i in 0..table_width {
        print!("-");
    }

    print!("+\n");

    for record in body.iter() {
        print!("| ");

        for i in 0..n_column {
            let len = column_width[i];
            print!("{:<len$}| ", record[i]);
        }

        println!();
    }

    print!("+");

    for _i in 0..table_width {
        print!("-");
    }

    print!("+\n");

    ExitCode::SUCCESS
}
