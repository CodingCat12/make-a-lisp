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

    assert_eq!(output.eval(), 15)
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

    assert_eq!(output.eval(), 10)
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

    assert_eq!(output.eval(), 15.0)
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

    assert!(!output.eval());

    let input = r#"
    ( || (
      true
      false
      true
    ))
    "#
    .trim_ascii();

    let (_, output) = bool::parse_expr(input).unwrap();

    assert!(output.eval())
}
