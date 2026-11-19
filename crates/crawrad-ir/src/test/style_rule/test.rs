//! Tests for the style rule expression parser and evaluator.

use crate::style_rule::{CompareOp, Expr, Literal, ParseError, parse};
use std::collections::HashMap;

fn mock_row<'a>(pairs: &'a [(&'a str, Option<&'a str>)]) -> impl Fn(&str) -> Option<String> + 'a {
    let map: HashMap<&str, Option<&str>> = pairs.iter().copied().collect();
    move |field: &str| {
        for (k, v) in &map {
            if k.eq_ignore_ascii_case(field) {
                return v.map(str::to_string);
            }
        }
        None
    }
}

#[test]
fn parse_and_eval_every_compare_operator() {
    // Eq (=)
    let expr = parse("balance = 100").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Compare {
            field: "balance".into(),
            op: CompareOp::Eq,
            value: Literal::Number(100.0),
        }
    );
    let row1 = mock_row(&[("balance", Some("100"))]);
    let row2 = mock_row(&[("balance", Some("99"))]);
    assert!(expr.eval(&row1));
    assert!(!expr.eval(&row2));

    // Ne (<>)
    let expr = parse("status <> \"B\"").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Compare {
            field: "status".into(),
            op: CompareOp::Ne,
            value: Literal::Text("B".into()),
        }
    );
    let row_a = mock_row(&[("status", Some("A"))]);
    let row_b = mock_row(&[("status", Some("B"))]);
    assert!(expr.eval(&row_a));
    assert!(!expr.eval(&row_b));

    // Lt (<) with negative number
    let expr = parse("balance < -1.5").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Compare {
            field: "balance".into(),
            op: CompareOp::Lt,
            value: Literal::Number(-1.5),
        }
    );
    assert!(expr.eval(&mock_row(&[("balance", Some("-2.0"))])));
    assert!(!expr.eval(&mock_row(&[("balance", Some("-1.5"))])));
    assert!(!expr.eval(&mock_row(&[("balance", Some("0"))])));

    // Le (<=)
    let expr = parse("score <= 50").expect("valid expression");
    assert!(expr.eval(&mock_row(&[("score", Some("50"))])));
    assert!(expr.eval(&mock_row(&[("score", Some("49.9"))])));
    assert!(!expr.eval(&mock_row(&[("score", Some("50.1"))])));

    // Gt (>)
    let expr = parse("score > 50").expect("valid expression");
    assert!(expr.eval(&mock_row(&[("score", Some("50.1"))])));
    assert!(!expr.eval(&mock_row(&[("score", Some("50"))])));

    // Ge (>=)
    let expr = parse("balance >= 10000").expect("valid expression");
    assert!(expr.eval(&mock_row(&[("balance", Some("10000"))])));
    assert!(expr.eval(&mock_row(&[("balance", Some("10001"))])));
    assert!(!expr.eval(&mock_row(&[("balance", Some("9999"))])));

    // Contains
    let expr = parse("city contains \"York\"").expect("valid expression");
    assert!(expr.eval(&mock_row(&[("city", Some("New York City"))])));
    assert!(!expr.eval(&mock_row(&[("city", Some("Chicago"))])));
    assert!(!expr.eval(&mock_row(&[("city", Some("new york"))]))); // case-sensitive
}

#[test]
fn parse_and_eval_boolean_literals() {
    let expr = parse("active = true").expect("valid expression");
    assert!(expr.eval(&mock_row(&[("active", Some("true"))])));
    assert!(expr.eval(&mock_row(&[("active", Some("1"))])));
    assert!(!expr.eval(&mock_row(&[("active", Some("false"))])));
    assert!(!expr.eval(&mock_row(&[("active", Some("0"))])));

    let expr_false = parse("active = false").expect("valid expression");
    assert!(expr_false.eval(&mock_row(&[("active", Some("false"))])));
    assert!(expr_false.eval(&mock_row(&[("active", Some("0"))])));
    assert!(!expr_false.eval(&mock_row(&[("active", Some("true"))])));
}

#[test]
fn parse_and_eval_is_null_and_is_not_null() {
    let expr_null = parse("city is null").expect("valid expression");
    assert_eq!(
        expr_null,
        Expr::IsNull {
            field: "city".into(),
            negated: false,
        }
    );
    assert!(expr_null.eval(&mock_row(&[("city", None)])));
    assert!(expr_null.eval(&mock_row(&[]))); // missing field counts as null
    assert!(!expr_null.eval(&mock_row(&[("city", Some("Berlin"))])));

    let expr_not_null = parse("city is not null").expect("valid expression");
    assert_eq!(
        expr_not_null,
        Expr::IsNull {
            field: "city".into(),
            negated: true,
        }
    );
    assert!(expr_not_null.eval(&mock_row(&[("city", Some("Berlin"))])));
    assert!(!expr_not_null.eval(&mock_row(&[("city", None)])));
    assert!(!expr_not_null.eval(&mock_row(&[])));
}

#[test]
fn parse_and_eval_precedence_and_parentheses() {
    // and binds tighter than or: a or b and c == a or (b and c)
    let expr = parse("status = \"B\" or balance < 0 and city is null").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Or(
            Box::new(Expr::Compare {
                field: "status".into(),
                op: CompareOp::Eq,
                value: Literal::Text("B".into()),
            }),
            Box::new(Expr::And(
                Box::new(Expr::Compare {
                    field: "balance".into(),
                    op: CompareOp::Lt,
                    value: Literal::Number(0.0),
                }),
                Box::new(Expr::IsNull {
                    field: "city".into(),
                    negated: false,
                }),
            )),
        )
    );

    // Parentheses change precedence: (a or b) and c
    let expr_paren =
        parse("(status = \"B\" or balance < 0) and city is null").expect("valid expression");
    assert_eq!(
        expr_paren,
        Expr::And(
            Box::new(Expr::Or(
                Box::new(Expr::Compare {
                    field: "status".into(),
                    op: CompareOp::Eq,
                    value: Literal::Text("B".into()),
                }),
                Box::new(Expr::Compare {
                    field: "balance".into(),
                    op: CompareOp::Lt,
                    value: Literal::Number(0.0),
                }),
            )),
            Box::new(Expr::IsNull {
                field: "city".into(),
                negated: false,
            }),
        )
    );

    // Check fields extraction
    let fields = expr_paren.fields();
    assert_eq!(fields, vec!["status", "balance", "city"]);
}

#[test]
fn eval_missing_field_and_number_vs_text() {
    let expr = parse("balance < 0").expect("valid expression");
    // Missing field evaluates to false
    assert!(!expr.eval(&mock_row(&[])));

    // Non-numeric text in cell compared against number literal evaluates to false
    assert!(!expr.eval(&mock_row(&[("balance", Some("not_a_number"))])));

    // Lexical comparison for text literals
    let expr_text = parse("code < \"M\"").expect("valid expression");
    assert!(expr_text.eval(&mock_row(&[("code", Some("A"))])));
    assert!(!expr_text.eval(&mock_row(&[("code", Some("Z"))])));
}

#[test]
fn four_error_messages_are_human_readable() {
    // 1. expected a comparison after "balance"
    let err = parse("balance").unwrap_err();
    assert_eq!(
        err,
        ParseError {
            at: 7,
            message: "expected a comparison after \"balance\"".to_string(),
        }
    );

    let err2 = parse("balance and x = 1").unwrap_err();
    assert_eq!(
        err2,
        ParseError {
            at: 7,
            message: "expected a comparison after \"balance\"".to_string(),
        }
    );

    // 2. unclosed (
    let err = parse("(balance < 0").unwrap_err();
    assert_eq!(
        err,
        ParseError {
            at: 0,
            message: "unclosed (".to_string(),
        }
    );

    // 3. unknown operator "=="
    let err = parse("balance == 0").unwrap_err();
    assert_eq!(
        err,
        ParseError {
            at: 8,
            message: "unknown operator \"==\"".to_string(),
        }
    );

    // 4. expected a value after "<"
    let err = parse("balance <").unwrap_err();
    assert_eq!(
        err,
        ParseError {
            at: 9,
            message: "expected a value after \"<\"".to_string(),
        }
    );
}

#[test]
fn text_literal_keeps_non_ascii_and_escapes() {
    let expr = parse("city = \"São Paulo\"").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Compare {
            field: "city".into(),
            op: CompareOp::Eq,
            value: Literal::Text("São Paulo".into()),
        }
    );
    assert!(expr.eval(&mock_row(&[("city", Some("São Paulo"))])));

    let expr = parse("note contains \"a\\\"b\n\"").expect("valid expression");
    assert_eq!(
        expr,
        Expr::Compare {
            field: "note".into(),
            op: CompareOp::Contains,
            value: Literal::Text("a\"b\n".into()),
        }
    );
}
