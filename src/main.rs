#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod ast;

fn main() -> Result<(), std::io::Error> {
    return Ok(());
}

#[test]
fn expression_parser() {
    assert!(grammar::ExprParser::new().parse("22").is_ok());
    assert!(grammar::ExprParser::new().parse("22 + 22").is_ok());
    assert!(grammar::ExprParser::new().parse("22 * 22").is_ok());
    assert!(grammar::ExprParser::new().parse("a + a * a").is_ok());
    assert!(grammar::ExprParser::new().parse("a * a + a").is_ok());
    assert!(grammar::ExprParser::new().parse("aAaAa").is_ok());
    assert!(grammar::ExprParser::new().parse("(22)").is_ok());

    assert!(grammar::ExprParser::new().parse("22 * + 22").is_err());
    assert!(grammar::ExprParser::new().parse("22.1").is_err());
    assert!(grammar::ExprParser::new().parse("a A").is_err());
    assert!(grammar::ExprParser::new().parse("a1 + a").is_err());
}

#[test]
fn logical() {
    assert!(grammar::LogicalExprParser::new().parse("22 < 21").is_ok());
    assert!(grammar::LogicalExprParser::new().parse("!22 < 22").is_ok());
    assert!(grammar::LogicalExprParser::new().parse("!!22 < 22").is_ok());
    assert!(grammar::LogicalExprParser::new()
        .parse("!22 < 22 && 22 < 22")
        .is_ok());

    assert!(grammar::LogicalExprParser::new()
        .parse("!(22 < 22 || 22 < 22)")
        .is_ok());

    assert!(grammar::LogicalExprParser::new().parse("22 < !22").is_err());
    assert!(grammar::LogicalExprParser::new()
        .parse("22 < 22 < 22")
        .is_err());
}
