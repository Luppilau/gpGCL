use crate::ast;

pub trait Visitor: Sized {
    fn visit_expr(&mut self, expr: &mut ast::Expr) {
        walk_expr(self, expr)
    }

    fn visit_expr_opcode(&mut self, opcode: &mut ast::ExprOpcode) {}

    fn visit_command(&mut self, command: &mut ast::Command) {
        walk_command(self, command)
    }

    fn visit_logical_expr(&mut self, expr: &mut ast::LogicalExpr) {
        walk_logical_expr(self, expr)
    }
    fn visit_logical_expr_opcode(&mut self, opcode: &mut ast::LogicalExprOpcode) {}

    fn visit_number(&mut self, number: &mut f32) {}

    fn visit_variable(&mut self, variable: &mut String) {}

    fn visit_probability(&mut self, probability: &mut ast::Probability) {}

    fn visit_probability_distribution(&mut self, distribution: &mut ast::ProbabilityDistribution) {}

    fn visit_skip(&mut self) {}

    fn visit_diverge(&mut self) {}

    fn visit_tick(&mut self, tick: &mut ast::Command::Tick) {}

    fn visit_assignment(&mut self, assignment: &mut ast::Assignment) {}

    fn visit_random_assignment(&mut self, assignment: &mut ast::RandomAssignment) {}

    fn visit_sequence(&mut self, sequence: &mut ast::Sequence) {}

    fn visit_nondeterministic_choice(&mut self, choice: &mut ast::NondeterministicChoice) {}

    fn visit_probabilistic_choice(&mut self, choice: &mut ast::ProbabilisticChoice) {}

    fn visit_if(&mut self, if_: &mut ast::If) {}

    fn visit_if_else(&mut self, if_else: &mut ast::IfElse) {}

    fn visit_while(&mut self, while_: &mut ast::While) {}
}

pub fn walk_expr<V: Visitor + Sized>(visitor: &mut V, expr: &mut ast::Expr) {
    match expr {
        ast::Expr::Number(n) => visitor.visit_number(n),
        ast::Expr::Variable(x) => visitor.visit_variable(x),
        ast::Expr::ExprOp(l, op, r) => {
            visitor.visit_expr(l);
            visitor.visit_expr_opcode(op);
            visitor.visit_expr(r);
        }
    }
}

pub fn walk_command<V: Visitor + Sized>(visitor: &mut V, command: &mut ast::Command) {
    match command {
        ast::Command::Skip => visitor.visit_skip(),
        ast::Command::Diverge => visitor.visit_diverge(),
        ast::Command::Tick(tick) => visitor.visit_tick(tick),
        ast::Command::Assignment(x, expr) => {
            visitor.visit_variable(x);
            visitor.visit_expr(expr);
        }
        ast::Command::RandomAssignment(x, distribution) => {
            visitor.visit_variable(x);
            visitor.visit_probability_distribution(distribution);
        }
        ast::Command::Sequence(l, r) => {
            visitor.visit_command(l);
            visitor.visit_command(r);
        }
        ast::Command::NondeterministicChoice(l, r) => {
            visitor.visit_command(l);
            visitor.visit_command(r);
        }
        ast::Command::ProbabilisticChoice(l, probability, r) => {
            visitor.visit_command(l);
            visitor.visit_probability(probability);
            visitor.visit_command(r);
        }
        ast::Command::If(expr, command) => {
            visitor.visit_logical_expr(expr);
            visitor.visit_command(command);
        }
        ast::Command::IfElse(expr, command, else_command) => {
            visitor.visit_logical_expr(expr);
            visitor.visit_command(command);
            visitor.visit_command(else_command);
        }
        ast::Command::While(expr, command) => {
            visitor.visit_logical_expr(expr);
            visitor.visit_command(command);
        }
    }
}

pub fn walk_logical_expr<V: Visitor + Sized>(visitor: &mut V, expr: &mut ast::LogicalExpr) {}
