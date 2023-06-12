#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod ast;
mod transformer;
mod visitor;

use transformer::Transformable;

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

    let mut transformer = transformer::AstTransformer::new();

    println!("Original AST: \n{ast}");

    // transformer.set_expr_transformer(|x| {
    //     let result: &mut ast::Expr = match x {
    //         ast::Expr::Number(val) => {
    //             let new_val = *val + 1.0;
    //             let expr = ast::Expr::Number(new_val);
    //             let expr_ref = Box::leak(Box::new(expr));
    //             expr_ref
    //         }
    //         ast::Expr::Variable(_) => x,
    //         ast::Expr::ExprOp(l, op, r) => {
    //             println!("Transforming: {:?} {} {:?}", l, op, r);
    //             x
    //         }
    //     };

    //     result
    // });

    // let ast = ast.as_mut();
    // ast.transform(&mut transformer);

    println!("Transformed AST: \n{ast}");
}
