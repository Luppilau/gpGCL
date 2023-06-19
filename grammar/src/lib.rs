#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

pub mod ast;
pub mod transform;
pub mod transpile;
pub mod visit;

pub fn parse_grammar(input: &str) -> Result<ast::Command, String> {
    let parser = grammar::commandParser::new();
    match parser.parse(input) {
        Ok(command) => Ok(*command),
        Err(e) => Err(format!("{}", e)),
    }
}
