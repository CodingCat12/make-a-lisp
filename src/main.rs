use make_a_lisp::interpreter;

fn main() {
    let input = r#"
    ( avg (
      ( + 
        ( 5 5 )
      )
      ( *
        ( 2 10)
      )
    ))
    "#
    .trim_ascii();

    let output = interpreter::parse_expr(input).unwrap();

    println!("{:#?}", output.1);
}
