#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod ast;

fn main() -> Result<(), std::io::Error> {
    return Ok(());
}

#[test]
fn expression_parser() {
    let parser = grammar::exprParser::new();

    assert!(parser.parse("22").is_ok());
    assert!(parser.parse("variablename").is_ok());
    assert!(parser.parse("22 * 22").is_ok());
    assert!(parser.parse("22 / 22").is_ok());
    assert!(parser.parse("22 % 22").is_ok());
    assert!(parser.parse("22 + 22").is_ok());
    assert!(parser.parse("22 - 22").is_ok());
    assert!(parser.parse("22 :- 22").is_ok());
    assert!(parser.parse("a + a * a").is_ok());
    assert!(parser.parse("a * a + a").is_ok());
    assert!(parser.parse("(22)").is_ok());

    assert!(parser.parse("22 * + 22").is_err());
    assert!(parser.parse("22.1").is_err());
    assert!(parser.parse("a A").is_err());
    assert!(parser.parse("a1 + a").is_err());
}

#[test]
fn expression_associativity_parser() {
    let parser = grammar::exprParser::new();

    let expr = parser.parse("1 * 2 + 3").unwrap();
    assert_eq!(&format!("{:?}", expr), "((1 * 2) + 3)");

    let expr = parser.parse("1 / 2 - 3").unwrap();
    assert_eq!(&format!("{:?}", expr), "((1 / 2) - 3)");

    let expr = parser.parse("1 % 2 :- 3").unwrap();
    assert_eq!(&format!("{:?}", expr), "((1 % 2) :- 3)");

    let expr = parser.parse("1 * (2 + 3)").unwrap();
    assert_eq!(&format!("{:?}", expr), "(1 * (2 + 3))");
}

#[test]
fn logical() {
    let parser = grammar::logical_exprParser::new();

    assert!(parser.parse("!22 < 21").is_ok());
    assert!(parser.parse("1 < 1 && 1 < 1").is_ok());
    assert!(parser.parse("1 < 1 || 1 < 1").is_ok());
    assert!(parser.parse("22 < 21").is_ok());
    assert!(parser.parse("22 <= 21").is_ok());
    assert!(parser.parse("22 > 22").is_ok());
    assert!(parser.parse("22 >= 22").is_ok());
    assert!(parser.parse("22 == 22").is_ok());
    assert!(parser.parse("22 != 22").is_ok());

    assert!(parser.parse("22 !!= 22").is_err());
    assert!(parser.parse("22 < !22").is_err());
    assert!(parser.parse("22 < 22 < 22").is_err());
}

#[test]

fn logical_associativity_parser() {
    let parser = grammar::logical_exprParser::new();

    let expr = parser.parse("!1>2 && 1>2").unwrap();
    assert_eq!(&format!("{:?}", expr), "!((1 > 2) && (1 > 2))");

    let expr = parser.parse("(!1>2) && 1>2").unwrap();
    assert_eq!(&format!("{:?}", expr), "(!(1 > 2) && (1 > 2))");

    let expr = parser.parse("!(1>2) && (1>2)").unwrap();
    assert_eq!(&format!("{:?}", expr), "!((1 > 2) && (1 > 2))");

    let expr = parser.parse("1>2 && 1>2 || 1>2").unwrap();
    assert_eq!(&format!("{:?}", expr), "(((1 > 2) && (1 > 2)) || (1 > 2))");
}

#[test]
fn command() {
    let parser = grammar::commandParser::new();

    assert!(parser.parse("skip").is_ok());
    assert!(parser.parse("diverge").is_ok());
    assert!(parser.parse("tick(1)").is_ok());
    assert!(parser.parse("x:=1").is_ok());
    assert!(parser.parse("x := normal(1,2)").is_ok());
    assert!(parser.parse("x := uniform(1)").is_ok());
    assert!(parser.parse("x := lognormal(1)").is_ok());
    assert!(parser.parse("x := exponential(1)").is_ok());
    assert!(parser.parse("skip ; skip").is_ok());
    assert!(parser.parse("{skip} [] {skip}").is_ok());
    assert!(parser.parse("{skip} [0.1] {skip}").is_ok());
    assert!(parser.parse("if (a==0) { skip }").is_ok());
    assert!(parser.parse("if (a==0) { skip } else { skip }").is_ok());
    assert!(parser.parse("while (a > b) { skip }").is_ok());
    assert!(parser
        .parse(
            "
            if (a==0) { skip };
            if (a==0) { skip } else { skip }
            "
        )
        .is_ok());
    assert!(parser
        .parse(
            "if (x>=y) {
                {skip} [] {skip}
            } else {
                diverge
            }
            "
        )
        .is_ok());

    assert!(parser.parse("tick(0.1)").is_err());

    let expr = parser
        .parse(
            "
        if (0 == 0) {
            {skip; skip} [] {skip; skip}
        };
        {skip; skip} [0.2] {tick(1)};
        x:=exponential(1);
        while(a>b) {
            skip
        }
        ",
        )
        .unwrap();
    assert_eq!(&format!("{:?}", expr),"if (0 == 0) { {skip; skip} [] {skip; skip} }; {skip; skip} [0.2] {tick(1)}; \"x\" := exponential(1); while (\"a\" > \"b\") { skip }");
}
