use grammar::ast::*;
use grammar::transform::{self, Transform};
use grammar::transpile::{self, Transpile};
use grammar::visit::Visit;
// use unsupported::unsupported;

const BOUNDED_RETRANSMISSION: &str = "fail := 0;
sent := 0;
while (sent < 8000000 && fail < 10) {
    { fail := 0; sent := sent + 1} [0.999] {fail := fail + 1}
}; diverge";

fn main() {
    let parser = grammar::grammar::commandParser::new();
    let ast = parser.parse(BOUNDED_RETRANSMISSION).unwrap();

    struct SupportChecker {
        errors: Vec<String>,
    }
    impl SupportChecker {
        fn new() -> Self {
            Self { errors: vec![] }
        }
    }

    impl grammar::visit::Visit for SupportChecker {
        fn visit_diverge(&mut self, _i: &Diverge) {
            self.errors
                .push("Diverge operation not supported".to_string());
        }

        fn visit_skip(&mut self, _i: &Skip) {
            self.errors.push("Skip operation not supported".to_string());
        }
    }

    let mut checker = SupportChecker::new();
    checker.visit_command(&ast);
    if !checker.errors.is_empty() {
        for error in checker.errors {
            println!("{}", error);
        }
    }

    struct DeterminismChecker {
        is_deterministic: bool,
    }

    impl DeterminismChecker {
        fn new() -> Self {
            Self {
                is_deterministic: true,
            }
        }
    }

    impl grammar::visit::Visit for DeterminismChecker {
        fn visit_random_assignment(&mut self, _i: &RandomAssignment) {
            self.is_deterministic = false;
        }
        fn visit_nondeterministic_choice(&mut self, _i: &NondetChoice) {
            self.is_deterministic = false;
        }
        fn visit_probabilistic_choice(&mut self, _i: &ProbChoice) {
            self.is_deterministic = false;
        }
    }

    let mut checker = DeterminismChecker::new();
    checker.visit_command(&ast);

    struct Transformer;
    impl grammar::transform::Transform for Transformer {
        fn transform_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
            if let LogicalExprOpcode::GreaterThan = i.op {
                return LogicalExprOp {
                    left: Box::new(transform::transform_expr(self, *i.right)),
                    op: transform::transform_logical_expr_opcode(self, LogicalExprOpcode::LessThan),
                    right: Box::new(transform::transform_expr(self, *i.left)),
                };
            }

            transform::transform_logical_expr_op(self, i)
        }
    }

    struct Transpiler;
    impl grammar::transpile::Transpile for Transpiler {
        fn transpile_assignment(&mut self, i: Assignment) -> String {
            format!(
                "{} :=== {}",
                transpile::transpile_lit_variable(self, i.name),
                transpile::transpile_expr(self, *i.expr)
            )
        }
    }

    let ast = Transformer.transform_command(*ast);
    let ast = Transpiler.transpile_command(ast);

    println!("{}", ast);
}
