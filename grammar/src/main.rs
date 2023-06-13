#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod ast;

mod visitor;

fn main() {
    let parser = grammar::commandParser::new();

    let mut ast = parser
        .parse(
            "fail := 0;
            sent := 0;
            while (sent < 8000000 && fail < 10) {
                { fail := 0; sent := sent + 1} [0.999] {fail := fail + 1}
            }",
        )
        .unwrap();
}
