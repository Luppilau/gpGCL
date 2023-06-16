#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod ast;
mod fold;
mod transpile;
mod visit;

pub fn parse_grammar(input: &str) -> Result<ast::Command, String> {
    let parser = grammar::commandParser::new();
    match parser.parse(input) {
        Ok(command) => Ok(*command),
        Err(e) => Err(format!("{}", e)),
    }
}
