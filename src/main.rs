use make_a_lisp::interpreter;

fn main() {
    println!("Welcome to the REPL! Type 'exit' to quit.");

    let input = "( - (* (2 5)) 5)";
    match interpreter::parse_expr(input) {
        Ok((_, result)) => {
            println!("Result: {:?}", result);
        }
        Err(e) => eprintln!("Parsing Error: {:?}", e),
    }
}
