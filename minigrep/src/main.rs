use std::{env, process::exit};

use minigrep::_read_file_line_by_line;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not Enough Argument");
        println!("Help: cargo run -- <file_path> <text_to_search>");
        println!("Example: cargo run -- test_file.txt bittu");
        exit(0);
    }

    let file_path = &args[1];
    let text_to_search = &args[2];

    let res = _read_file_line_by_line(file_path);
    match res {
        Ok(content) => {
            println!("Searching file {}...", file_path);
            for (index, line) in content.iter().enumerate() {
                if line.contains(text_to_search) {
                    println!("Line {}: {}\n", index+1, line);
                }
            }
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
