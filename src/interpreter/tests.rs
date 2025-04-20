use super::*;

#[test]
fn basic_int() {
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

    let (_, output) = int::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), 15)
}

#[test]
fn complex_int() {
    let input = r#"
    ( -
      ( avg (
        ( + 
          ( 5 5 )
        )
          ( *
          ( 2 10)
        )
      ))
      5
    )
    "#
    .trim_ascii();

    let (_, output) = int::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), 10)
}

#[test]
fn basic_float() {
    let input = r#"
    ( avg (
      ( + 
        ( 10.0 7.5 )
      )
      ( *
        ( 1.25 10.0 )
      )
    ))
    "#
    .trim_ascii();

    let (_, output) = float::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), 15.0)
}

#[test]
fn basic_bool() {
    let input = r#"
    ( && (
      true
      false
      true
    ))
    "#
    .trim_ascii();

    let (_, output) = bool::parse_expr(input).unwrap();

    assert!(!output.eval(&Env::default()));

    let input = r#"
    ( || (
      true
      false
      true
    ))
    "#
    .trim_ascii();

    let (_, output) = bool::parse_expr(input).unwrap();

    assert!(output.eval(&Env::default()))
}

#[test]
fn string_join() {
    let input = r#"(+ ("hello" "world"))"#;
    let (_, output) = string::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), String::from("helloworld"));
}

#[test]
fn consts() {
    let input = "PI";
    let (_, output) = float::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), std::f64::consts::PI);

    let input = r#"
    ( + (
    HELLO
    "a"
    "b"
    ))"#
    .trim_ascii();
    let (_, output) = string::parse_expr(input).unwrap();

    assert_eq!(
        output.eval(&Env::default()),
        String::from("Hello, world!ab")
    )
}

#[test]
fn var() {
    let input = "( let x 5 (- 10 x) )";

    let (_, output) = int::parse_expr(input).unwrap();

    assert_eq!(output.eval(&Env::default()), 5)
}
