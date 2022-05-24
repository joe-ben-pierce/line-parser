
pub mod expr;
pub mod utils;

use crate::expr::Expr;
fn main(){
    test_segment();
    test_valids();
    //_test_invalids();
    println!("all tests pass");
}

fn _assert_eq(a: &Expr, b: &Expr){
    if a != b {
        println!("a == {}, b == {}", a.eval(), b.eval());
    }
}

pub fn test_case_eval(s: &str, expected: f64){
    let actual = match Expr::from_str(s){
        None => {
            println!("input : {}", s);
            println!("tokenized: {:?}", Expr::segment(s));
            panic!("recieved None but expected {}", expected)
        },
        Some(expr) => expr
    };
    if !utils::eq_ish(actual.eval(), expected){
        println!("input : {}", s);
        println!("expr was {:?}", actual);
        panic!("recieved {} but expected {}", actual.eval(), expected);
    }
}

pub fn test_case_section(s: &str, start: usize, end: usize, expected: f64){
    let tokens = &Expr::segment(s)[start..end];
    let actual = match Expr::from_segments(tokens){
        None => panic!("recieved None but expected {}", expected),
        Some(expr) => expr
    };
    if !utils::eq_ish(actual.eval(), expected){
        println!("expr was {:?}", actual);
        panic!("recieved {} but expected {}", actual.eval(), expected);
    }
}



pub fn test_valids(){
    test_case_eval("4", 4.0);
    test_case_eval("0", 0.0);
    test_case_eval("-1", -1.0);


    test_case_eval("1 + 2", 3.0);
    test_case_eval("1 + 4", 5.0);
    test_case_eval("1.3 + 2.0", 3.3);
    test_case_eval("1 + 2 + 3", 6.0);
    test_case_eval("1 + 2 + 3 + 4", 10.0);
    test_case_eval("1.1 + 2.2 + 3.456 + 7", 13.756);

    test_case_eval("2 * 3 + 1", 7.0);
    test_case_eval("3 * 4 * 5", 60.0);
    test_case_eval("2 + 3 + 4 * 5", 25.0);

    test_case_section("( 4 + 5 )", 1, 4, 9.0);
    test_case_section("( 4 * 5 )", 1, 4, 20.0);
    test_case_eval("( 1 + 2 )", 3.0);

    test_case_eval("( 3 * 6 )", 18.0);

    test_case_eval("1 + ( 2 * 3 )", 7.0);

    test_case_eval("( 1 + 2 ) * 3", 9.0);

    test_case_eval("( 1 + 2 ) * ( 3 + 4 )", 21.0);

    test_case_eval("( 1 + 2 ) * ( 7 - 9 )", -6.0);

    test_case_eval("( ( 3 - 4 ) )", -1.0);
    test_case_eval("( ( 1 + 2 ) + ( ( 3 * 4 ) - 9 ) ) * 4", 24.0);

    test_case_eval("(1+2+4)^3", 343.0);
    test_case_eval("1/4", 0.25);
    test_case_eval("(9-6)/-3", -1.0);
    test_case_eval("1+2*3", 7.0);
    test_case_eval("2*3+1", 7.0);

    test_case_eval("1 + 2 * 3 ^ 4 - 6", 157.0);
    test_case_eval("(1+2.3*4)-((5+6)^1.3)", -12.3845005502);
}

fn test_case_segment(s: &str, expected: Vec<&str>){
    let actual = Expr::segment(s);
    if actual != expected{
        panic!("expected {:?} but recieved {:?}", expected, actual);
    }
}

pub fn test_segment(){
    test_case_segment("1", vec!["1"]);
    test_case_segment("1 + 2", vec!["1", "+", "2"]);
    test_case_segment("1+2", vec!["1", "+", "2"]);
    test_case_segment("12 + 3", vec!["12", "+", "3"]);
    test_case_segment("123+456*789", vec!["123", "+", "456", "*", "789"]);
    test_case_segment("(1)", vec!["(", "1", ")"]);
    test_case_segment("(2 )", vec!["(", "2", ")"]);

    test_case_segment("(1+224 * 3) + 4 - 5 / 6", vec!["(", "1", "+", "224", "*", "3", ")", "+", "4", "-", "5", "/", "6"]);

    test_case_segment("-1 + 2", vec!["-1", "+", "2"]);

    test_case_segment("2 + -1", vec!["2", "+", "-1"]);
    test_case_segment("1 --2", vec!["1", "-", "-2"]);
    test_case_segment("(0.1 * -5.6) - 3", vec!["(", "0.1", "*", "-5.6", ")", "-", "3"]);

    test_case_segment("(1+2)^3", vec!["(", "1", "+", "2", ")", "^", "3"]);

    test_case_segment("12+3*4^-5", vec!["12", "+", "3", "*", "4", "^", "-5"]);
}


fn _assert_none_expr(s: &str){
    match Expr::from_str(s){
        None => (),
        Some(expr) => panic!("somehow it got the number {}", expr.eval())
    }
}

fn _test_invalids(){
    let lst = vec!["4 ++ 5", "1 *= 3", "a b c d e", "12 # 7"];
    for s in lst{
        _assert_none_expr(s);
    }
}
