use super::builtins::math::*;
use super::*;

#[test]
fn basic_expr() {
    let expr = Sum::new(vec![Box::new(1), Box::new(2), Box::new(3)]);
    let output: i32 = expr.eval(&Env::default());
    assert_eq!(output, 6)
}

#[test]
fn complex_expr() {
    let expr = Average::new(vec![
        Sum::new(vec![Box::new(2), Box::new(2)]),
        Box::new(2),
        Product::new(vec![Box::new(2), Box::new(4)]),
    ]);
    let output: i32 = expr.eval(&Env::default());
    assert_eq!(output, 4)
}
