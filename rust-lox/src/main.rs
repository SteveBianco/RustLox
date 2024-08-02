use std::env;
use std::fs;
use std::io;
use std::io::Write;

use rust_lox::scanner::Scanner;
use rust_lox::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() > 1 {
        let file_path: &str = parse_file_path(&args);
        dbg!(file_path);
        run_file(file_path);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &str) {
    let input = fs::read_to_string(file_path).expect("File does not exist");
    run(&input);
}

fn run_prompt() {
    // Continually process input from command line until user terminates witth ctrl-z.

    loop {
        // Print user prompt
        print!(">>> ");
        io::stdout().flush().expect("Expected to flush line.");

        let mut input: String = String::new();

        let nbytes = io::stdin()
            .read_line(&mut input)
            .expect("Expect to read from command line");

        if nbytes == 0 {
            break;
        }

        run(&input);
    }

    // Say good-bye
    println!("\nSession terminated by user.")
}

fn run(input: &String) {
    let _tokens = Scanner::scan(input);
    dbg!(_tokens);
}

fn parse_file_path(args: &[String]) -> &str {
    let file_path: &str = &args[1];
    return file_path;
}
