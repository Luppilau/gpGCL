use grammar::ast::*;
use grammar::transform::*;
use grammar::transpile::*;
use grammar::visit::*;

const _BOUNDED_TRANSMISSION: &str = "fail := 0;
sent := 0;
while (sent < 8000000 && fail < 10) {
    { fail := 0; sent := sent + 1} [0.999] {fail := fail + 1}
}";

fn main() {
    let ast = grammar::parse_grammar(_BOUNDED_TRANSMISSION).unwrap();

    struct SupportChecker {
        errors: Vec<String>,
    }
    impl grammar::visit::Visit for SupportChecker {
        fn visit_diverge(&mut self, _i: &Diverge) {
            self.errors
                .push("Diverge operation not supported".to_string());
        }
    }
    let mut checker = SupportChecker { errors: vec![] };
    checker.visit_command(&ast);
    if !checker.errors.is_empty() {
        for error in checker.errors {
            println!("{}", error);
        }
        return;
    }

    struct Transformer;
    impl grammar::transform::Transform for Transformer {
        fn transform_logical_op(&mut self, i: LogicalOp) -> LogicalOp {
            LogicalOp {
                left: i.right,
                op: i.op,
                right: i.left,
            }
        }
    }

    struct Transpiler;
    impl grammar::transpile::Transpile for Transpiler {
        fn transpile_assignment(&mut self, i: Assignment) -> String {
            format!(
                "{} :=== {}",
                transpile_lit_variable(self, i.name),
                transpile_expr(self, *i.expr)
            )
        }
    }

    let ast = Transformer.transform_command(ast);
    let ast = Transpiler.transpile_command(ast);

    println!("{}", ast);
}
