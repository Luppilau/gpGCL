use crate::ast::*;
#[allow(dead_code)]

pub trait Visit {
    fn visit_lit_number(&mut self, i: &LitNumber) {
        visit_lit_number(self, i)
    }
    fn visit_lit_variable(&mut self, i: &LitVariable) {
        visit_lit_variable(self, i)
    }
    fn visit_expr_op(&mut self, i: &ExprOp) {
        visit_expr_op(self, i)
    }
    fn visit_expr_opcode(&mut self, i: &ExprOpcode) {
        visit_expr_opcode(self, i)
    }
    fn visit_expr(&mut self, i: &Expr) {
        visit_expr(self, i)
    }
    fn visit_not(&mut self, i: &Not) {
        visit_not(self, i)
    }
    fn visit_logical_op(&mut self, i: &LogicalOp) {
        visit_logical_op(self, i)
    }
    fn visit_logical_expr_op(&mut self, i: &LogicalExprOp) {
        visit_logical_expr_op(self, i)
    }
    fn visit_logical_opcode(&mut self, i: &LogicalOpcode) {
        visit_logical_opcode(self, i)
    }
    fn visit_logical_expr_opcode(&mut self, i: &LogicalExprOpcode) {
        visit_logical_expr_opcode(self, i)
    }
    fn visit_logical_expr(&mut self, i: &LogicalExpr) {
        visit_logical_expr(self, i)
    }
    fn visit_skip(&mut self, i: &Skip) {
        visit_skip(self, i)
    }
    fn visit_diverge(&mut self, i: &Diverge) {
        visit_diverge(self, i)
    }
    fn visit_tick(&mut self, i: &Tick) {
        visit_tick(self, i)
    }
    fn visit_assignment(&mut self, i: &Assignment) {
        visit_assignment(self, i)
    }
    fn visit_random_assignment(&mut self, i: &RandomAssignment) {
        visit_random_assignment(self, i)
    }
    fn visit_sequence(&mut self, i: &Sequence) {
        visit_sequence(self, i)
    }
    fn visit_nondeterministic_choice(&mut self, i: &NondetChoice) {
        visit_nondeterministic_choice(self, i)
    }
    fn visit_probabilistic_choice(&mut self, i: &ProbChoice) {
        visit_probabilistic_choice(self, i)
    }
    fn visit_if(&mut self, i: &If) {
        visit_if(self, i)
    }
    fn visit_if_else(&mut self, i: &IfElse) {
        visit_if_else(self, i)
    }
    fn visit_while(&mut self, i: &While) {
        visit_while(self, i)
    }
    fn visit_command(&mut self, i: &Command) {
        visit_command(self, i)
    }
    fn visit_lit_prob(&mut self, i: &LitProb) {
        visit_lit_prob(self, i)
    }
    fn visit_prob_dist(&mut self, i: &ProbDist) {
        visit_prob_dist(self, i)
    }
    fn visit_prob_dist_normal(&mut self, i: &ProbDistNormal) {
        visit_prob_dist_normal(self, i)
    }
    fn visit_prob_dist_uniform(&mut self, i: &ProbDistUniform) {
        visit_prob_dist_uniform(self, i)
    }
    fn visit_prob_dist_log_normal(&mut self, i: &ProbDistLogNormal) {
        visit_prob_dist_log_normal(self, i)
    }
    fn visit_prob_dist_exponential(&mut self, i: &ProbDistExponential) {
        visit_prob_dist_exponential(self, i)
    }
}

pub fn visit_lit_number<V>(_v: &mut V, _i: &LitNumber)
where
    V: Visit + ?Sized,
{
}

pub fn visit_lit_variable<V>(_v: &mut V, _i: &LitVariable)
where
    V: Visit + ?Sized,
{
}

pub fn visit_expr_op<V>(v: &mut V, i: &ExprOp)
where
    V: Visit + ?Sized,
{
    v.visit_expr(&i.left);
    v.visit_expr_opcode(&i.op);
    v.visit_expr(&i.right);
}

pub fn visit_expr_opcode<V>(_v: &mut V, _ii: &ExprOpcode)
where
    V: Visit + ?Sized,
{
}

pub fn visit_expr<V>(v: &mut V, i: &Expr)
where
    V: Visit + ?Sized,
{
    match i {
        Expr::Number(i) => v.visit_lit_number(i),
        Expr::Variable(i) => v.visit_lit_variable(i),
        Expr::ExprOp(i) => v.visit_expr_op(i),
    }
}

pub fn visit_not<V>(v: &mut V, i: &Not)
where
    V: Visit + ?Sized,
{
    v.visit_logical_expr(&i.expr);
}

pub fn visit_logical_op<V>(v: &mut V, i: &LogicalOp)
where
    V: Visit + ?Sized,
{
    v.visit_logical_expr(&i.left);
    v.visit_logical_opcode(&i.op);
    v.visit_logical_expr(&i.right);
}

pub fn visit_logical_expr_op<V>(v: &mut V, i: &LogicalExprOp)
where
    V: Visit + ?Sized,
{
    v.visit_expr(&i.left);
    v.visit_logical_expr_opcode(&i.op);
    v.visit_expr(&i.right);
}

pub fn visit_logical_opcode<V>(_v: &mut V, _i: &LogicalOpcode)
where
    V: Visit + ?Sized,
{
}

pub fn visit_logical_expr_opcode<V>(_v: &mut V, _i: &LogicalExprOpcode)
where
    V: Visit + ?Sized,
{
}

pub fn visit_logical_expr<V>(v: &mut V, i: &LogicalExpr)
where
    V: Visit + ?Sized,
{
    match i {
        LogicalExpr::Not(i) => v.visit_not(i),
        LogicalExpr::LogicalOp(i) => v.visit_logical_op(i),
        LogicalExpr::LogicalExprOp(i) => v.visit_logical_expr_op(i),
    }
}

pub fn visit_skip<V>(_v: &mut V, _i: &Skip)
where
    V: Visit + ?Sized,
{
}

pub fn visit_diverge<V>(_v: &mut V, _i: &Diverge)
where
    V: Visit + ?Sized,
{
}

pub fn visit_tick<V>(_v: &mut V, _i: &Tick)
where
    V: Visit + ?Sized,
{
}

pub fn visit_assignment<V>(v: &mut V, i: &Assignment)
where
    V: Visit + ?Sized,
{
    v.visit_lit_variable(&i.name);
    v.visit_expr(&i.expr);
}

pub fn visit_random_assignment<V>(v: &mut V, i: &RandomAssignment)
where
    V: Visit + ?Sized,
{
    v.visit_lit_variable(&i.name);
    v.visit_prob_dist(&i.distribution);
}

pub fn visit_sequence<V>(v: &mut V, i: &Sequence)
where
    V: Visit + ?Sized,
{
    v.visit_command(&i.left);
    v.visit_command(&i.right);
}

pub fn visit_nondeterministic_choice<V>(v: &mut V, i: &NondetChoice)
where
    V: Visit + ?Sized,
{
    v.visit_command(&i.left);
    v.visit_command(&i.right);
}

pub fn visit_probabilistic_choice<V>(v: &mut V, i: &ProbChoice)
where
    V: Visit + ?Sized,
{
    v.visit_command(&i.left);
    v.visit_lit_prob(&i.prob);
    v.visit_command(&i.right);
}

pub fn visit_if<V>(v: &mut V, i: &If)
where
    V: Visit + ?Sized,
{
    v.visit_logical_expr(&i.condition);
    v.visit_command(&i.command);
}

pub fn visit_if_else<V>(v: &mut V, i: &IfElse)
where
    V: Visit + ?Sized,
{
    v.visit_logical_expr(&i.condition);
    v.visit_command(&i.left);
    v.visit_command(&i.right);
}

pub fn visit_while<V>(v: &mut V, i: &While)
where
    V: Visit + ?Sized,
{
    v.visit_logical_expr(&i.condition);
    v.visit_command(&i.command);
}

pub fn visit_command<V>(v: &mut V, i: &Command)
where
    V: Visit + ?Sized,
{
    match i {
        Command::Skip(i) => v.visit_skip(i),
        Command::Diverge(i) => v.visit_diverge(i),
        Command::Tick(i) => v.visit_tick(i),
        Command::Assignment(i) => v.visit_assignment(i),
        Command::RandomAssignment(i) => v.visit_random_assignment(i),
        Command::Sequence(i) => v.visit_sequence(i),
        Command::NondetChoice(i) => v.visit_nondeterministic_choice(i),
        Command::ProbChoice(i) => v.visit_probabilistic_choice(i),
        Command::If(i) => v.visit_if(i),
        Command::IfElse(i) => v.visit_if_else(i),
        Command::While(i) => v.visit_while(i),
    }
}

pub fn visit_lit_prob<V>(v: &mut V, i: &LitProb)
where
    V: Visit + ?Sized,
{
    v.visit_lit_number(&i.value);
}

pub fn visit_prob_dist<V>(v: &mut V, i: &ProbDist)
where
    V: Visit + ?Sized,
{
    match i {
        ProbDist::Normal(i) => v.visit_prob_dist_normal(i),
        ProbDist::Uniform(i) => v.visit_prob_dist_uniform(i),
        ProbDist::LogNormal(i) => v.visit_prob_dist_log_normal(i),
        ProbDist::Exponential(i) => v.visit_prob_dist_exponential(i),
    }
}

pub fn visit_prob_dist_normal<V>(v: &mut V, i: &ProbDistNormal)
where
    V: Visit + ?Sized,
{
    v.visit_lit_number(&i.mean);
    v.visit_lit_number(&i.std_dev);
}

pub fn visit_prob_dist_uniform<V>(v: &mut V, i: &ProbDistUniform)
where
    V: Visit + ?Sized,
{
    v.visit_lit_number(&i.min);
    v.visit_lit_number(&i.max);
}

pub fn visit_prob_dist_log_normal<V>(v: &mut V, i: &ProbDistLogNormal)
where
    V: Visit + ?Sized,
{
    v.visit_lit_number(&i.mean);
    v.visit_lit_number(&i.std_dev);
}
pub fn visit_prob_dist_exponential<V>(v: &mut V, i: &ProbDistExponential)
where
    V: Visit + ?Sized,
{
    v.visit_lit_number(&i.lambda);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar;

    #[test]
    fn test_visit() {
        let parser = grammar::commandParser::new();
        let expr = parser
            .parse(
                "a := 1;
                b := normal(1,2);
                c := a + b;
                
                if (!a > c && b < c) {
                    { a:=1 } [] { a:=2 };
                    { a:=1 } [0.5] { a:=2 }
                } else {
                    skip;
                    tick(1)
                };
                
                while (1>2) {
                    diverge
                }",
            )
            .unwrap();

        #[derive(Default, Debug)]
        struct ExpressionCounter {
            lit_number_count: usize,
            lit_variable_count: usize,
            expr_op_count: usize,
            expr_opcode_count: usize,
            expr_count: usize,
            not_count: usize,
            logical_op_count: usize,
            logical_expr_op_count: usize,
            logical_opcode_count: usize,
            logical_expr_opcode_count: usize,
            logical_expr_count: usize,
            skip_count: usize,
            diverge_count: usize,
            tick_count: usize,
            assignment_count: usize,
            random_assignment_count: usize,
            sequence_count: usize,
            nondeterministic_choice_count: usize,
            probabilistic_choice_count: usize,
            if_count: usize,
            if_else_count: usize,
            while_count: usize,
            command_count: usize,
            lit_prob_count: usize,
            prob_dist_count: usize,
            prob_dist_normal_count: usize,
            prob_dist_uniform_count: usize,
            prob_dist_log_normal_count: usize,
            prob_dist_exponential_count: usize,
        }

        impl Visit for ExpressionCounter {
            fn visit_lit_number(&mut self, i: &LitNumber) {
                self.lit_number_count += 1;
                visit_lit_number(self, i)
            }
            fn visit_lit_variable(&mut self, i: &LitVariable) {
                self.lit_variable_count += 1;
                visit_lit_variable(self, i)
            }
            fn visit_expr_op(&mut self, i: &ExprOp) {
                self.expr_op_count += 1;
                visit_expr_op(self, i)
            }
            fn visit_expr_opcode(&mut self, i: &ExprOpcode) {
                self.expr_opcode_count += 1;
                visit_expr_opcode(self, i)
            }
            fn visit_expr(&mut self, i: &Expr) {
                self.expr_count += 1;
                visit_expr(self, i)
            }
            fn visit_not(&mut self, i: &Not) {
                self.not_count += 1;
                visit_not(self, i)
            }
            fn visit_logical_op(&mut self, i: &LogicalOp) {
                self.logical_op_count += 1;
                visit_logical_op(self, i)
            }
            fn visit_logical_expr_op(&mut self, i: &LogicalExprOp) {
                self.logical_expr_op_count += 1;
                visit_logical_expr_op(self, i)
            }
            fn visit_logical_opcode(&mut self, i: &LogicalOpcode) {
                self.logical_opcode_count += 1;
                visit_logical_opcode(self, i)
            }
            fn visit_logical_expr_opcode(&mut self, i: &LogicalExprOpcode) {
                self.logical_expr_opcode_count += 1;
                visit_logical_expr_opcode(self, i)
            }
            fn visit_logical_expr(&mut self, i: &LogicalExpr) {
                self.logical_expr_count += 1;
                visit_logical_expr(self, i)
            }
            fn visit_skip(&mut self, i: &Skip) {
                self.skip_count += 1;
                visit_skip(self, i)
            }
            fn visit_diverge(&mut self, i: &Diverge) {
                self.diverge_count += 1;
                visit_diverge(self, i)
            }
            fn visit_tick(&mut self, i: &Tick) {
                self.tick_count += 1;
                visit_tick(self, i)
            }
            fn visit_assignment(&mut self, i: &Assignment) {
                self.assignment_count += 1;
                visit_assignment(self, i)
            }
            fn visit_random_assignment(&mut self, i: &RandomAssignment) {
                self.random_assignment_count += 1;
                visit_random_assignment(self, i)
            }
            fn visit_sequence(&mut self, i: &Sequence) {
                self.sequence_count += 1;
                visit_sequence(self, i)
            }
            fn visit_nondeterministic_choice(&mut self, i: &NondetChoice) {
                self.nondeterministic_choice_count += 1;
                visit_nondeterministic_choice(self, i)
            }
            fn visit_probabilistic_choice(&mut self, i: &ProbChoice) {
                self.probabilistic_choice_count += 1;
                visit_probabilistic_choice(self, i)
            }
            fn visit_if(&mut self, i: &If) {
                self.if_count += 1;
                visit_if(self, i)
            }
            fn visit_if_else(&mut self, i: &IfElse) {
                self.if_else_count += 1;
                visit_if_else(self, i)
            }
            fn visit_while(&mut self, i: &While) {
                self.while_count += 1;
                visit_while(self, i)
            }
            fn visit_command(&mut self, i: &Command) {
                self.command_count += 1;
                visit_command(self, i)
            }
            fn visit_lit_prob(&mut self, i: &LitProb) {
                self.lit_prob_count += 1;
                visit_lit_prob(self, i)
            }
            fn visit_prob_dist(&mut self, i: &ProbDist) {
                self.prob_dist_count += 1;
                visit_prob_dist(self, i)
            }
            fn visit_prob_dist_normal(&mut self, i: &ProbDistNormal) {
                self.prob_dist_normal_count += 1;
                visit_prob_dist_normal(self, i)
            }
            fn visit_prob_dist_uniform(&mut self, i: &ProbDistUniform) {
                self.prob_dist_uniform_count += 1;
                visit_prob_dist_uniform(self, i)
            }
            fn visit_prob_dist_log_normal(&mut self, i: &ProbDistLogNormal) {
                self.prob_dist_log_normal_count += 1;
                visit_prob_dist_log_normal(self, i)
            }
            fn visit_prob_dist_exponential(&mut self, i: &ProbDistExponential) {
                self.prob_dist_exponential_count += 1;
                visit_prob_dist_exponential(self, i)
            }
        }

        let mut visitor = ExpressionCounter::default();
        visitor.visit_command(&expr);

        assert_eq!(visitor.lit_number_count, 10);
        assert_eq!(visitor.lit_variable_count, 13);
        assert_eq!(visitor.expr_op_count, 1);
        assert_eq!(visitor.expr_opcode_count, 1);
        assert_eq!(visitor.expr_count, 14);
        assert_eq!(visitor.not_count, 1);
        assert_eq!(visitor.logical_op_count, 1);
        assert_eq!(visitor.logical_expr_op_count, 3);
        assert_eq!(visitor.logical_opcode_count, 1);
        assert_eq!(visitor.logical_expr_opcode_count, 3);
        assert_eq!(visitor.logical_expr_count, 5);
        assert_eq!(visitor.skip_count, 1);
        assert_eq!(visitor.diverge_count, 1);
        assert_eq!(visitor.tick_count, 1);
        assert_eq!(visitor.assignment_count, 6);
        assert_eq!(visitor.random_assignment_count, 1);
        assert_eq!(visitor.sequence_count, 6);
        assert_eq!(visitor.nondeterministic_choice_count, 1);
        assert_eq!(visitor.probabilistic_choice_count, 1);
        assert_eq!(visitor.if_count, 0);
        assert_eq!(visitor.if_else_count, 1);
        assert_eq!(visitor.while_count, 1);
        assert_eq!(visitor.command_count, 20);
        assert_eq!(visitor.lit_prob_count, 1);
        assert_eq!(visitor.prob_dist_count, 1);
        assert_eq!(visitor.prob_dist_normal_count, 1);
        assert_eq!(visitor.prob_dist_uniform_count, 0);
        assert_eq!(visitor.prob_dist_log_normal_count, 0);
        assert_eq!(visitor.prob_dist_exponential_count, 0);
    }
}
