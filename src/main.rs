use make_a_lisp::interpreter;

use std::io::{self, Write};

fn main() {
    println!("Welcome to the REPL! Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == ":exit" {
            println!("Goodbye!");
            break;
        }

        match interpreter::parse_expr(input) {
            Ok((_, result)) => {
                println!("Result: {:?}", result);
            }
            Err(e) => eprintln!("Parsing Error: {:?}", e),
        }
    }
}
