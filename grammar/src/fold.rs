use crate::ast::*;
#[allow(dead_code)]

pub trait Fold {
    fn fold_lit_number(&mut self, i: LitNumber) -> LitNumber {
        fold_lit_number(self, i)
    }
    fn fold_lit_variable(&mut self, i: LitVariable) -> LitVariable {
        fold_lit_variable(self, i)
    }
    fn fold_expr_op(&mut self, i: ExprOp) -> ExprOp {
        fold_expr_op(self, i)
    }
    fn fold_expr_opcode(&mut self, i: ExprOpcode) -> ExprOpcode {
        fold_expr_opcode(self, i)
    }
    fn fold_expr(&mut self, i: Expr) -> Expr {
        fold_expr(self, i)
    }
    fn fold_not(&mut self, i: Not) -> Not {
        fold_not(self, i)
    }
    fn fold_logical_op(&mut self, i: LogicalOp) -> LogicalOp {
        fold_logical_op(self, i)
    }
    fn fold_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
        fold_logical_expr_op(self, i)
    }
    fn fold_logical_opcode(&mut self, i: LogicalOpcode) -> LogicalOpcode {
        fold_logical_opcode(self, i)
    }
    fn fold_logical_expr_opcode(&mut self, i: LogicalExprOpcode) -> LogicalExprOpcode {
        fold_logical_expr_opcode(self, i)
    }
    fn fold_logical_expr(&mut self, i: LogicalExpr) -> LogicalExpr {
        fold_logical_expr(self, i)
    }
    fn fold_skip(&mut self, i: Skip) -> Skip {
        fold_skip(self, i)
    }
    fn fold_diverge(&mut self, i: Diverge) -> Diverge {
        fold_diverge(self, i)
    }
    fn fold_tick(&mut self, i: Tick) -> Tick {
        fold_tick(self, i)
    }
    fn fold_assignment(&mut self, i: Assignment) -> Assignment {
        fold_assignment(self, i)
    }
    fn fold_random_assignment(&mut self, i: RandomAssignment) -> RandomAssignment {
        fold_random_assignment(self, i)
    }
    fn fold_sequence(&mut self, i: Sequence) -> Sequence {
        fold_sequence(self, i)
    }
    fn fold_nondeterministic_choice(&mut self, i: NondetChoice) -> NondetChoice {
        fold_nondeterministic_choice(self, i)
    }
    fn fold_probabilistic_choice(&mut self, i: ProbChoice) -> ProbChoice {
        fold_probabilistic_choice(self, i)
    }
    fn fold_if(&mut self, i: If) -> If {
        fold_if(self, i)
    }
    fn fold_if_else(&mut self, i: IfElse) -> IfElse {
        fold_if_else(self, i)
    }
    fn fold_while(&mut self, i: While) -> While {
        fold_while(self, i)
    }
    fn fold_command(&mut self, i: Command) -> Command {
        fold_command(self, i)
    }

    fn fold_lit_prob(&mut self, i: LitProb) -> LitProb {
        fold_lit_prob(self, i)
    }
    fn fold_prob_dist(&mut self, i: ProbDist) -> ProbDist {
        fold_prob_dist(self, i)
    }
    fn fold_prob_dist_normal(&mut self, i: ProbDistNormal) -> ProbDistNormal {
        fold_prob_dist_normal(self, i)
    }
    fn fold_prob_dist_uniform(&mut self, i: ProbDistUniform) -> ProbDistUniform {
        fold_prob_dist_uniform(self, i)
    }
    fn fold_prob_dist_log_normal(&mut self, i: ProbDistLogNormal) -> ProbDistLogNormal {
        fold_prob_dist_log_normal(self, i)
    }
    fn fold_prob_dist_exponential(&mut self, i: ProbDistExponential) -> ProbDistExponential {
        fold_prob_dist_exponential(self, i)
    }
}

pub fn fold_lit_number<V>(_v: &mut V, i: LitNumber) -> LitNumber
where
    V: Fold + ?Sized,
{
    LitNumber { ..i }
}
pub fn fold_lit_variable<V>(_v: &mut V, i: LitVariable) -> LitVariable
where
    V: Fold + ?Sized,
{
    LitVariable { ..i }
}
pub fn fold_expr_op<V>(v: &mut V, i: ExprOp) -> ExprOp
where
    V: Fold + ?Sized,
{
    ExprOp {
        op: v.fold_expr_opcode(i.op),
        left: Box::new(v.fold_expr(*i.left)),
        right: Box::new(v.fold_expr(*i.right)),
    }
}
pub fn fold_expr_opcode<V>(_v: &mut V, i: ExprOpcode) -> ExprOpcode
where
    V: Fold + ?Sized,
{
    i.clone()
}
pub fn fold_expr<V>(v: &mut V, i: Expr) -> Expr
where
    V: Fold + ?Sized,
{
    match i {
        Expr::Number(i) => Expr::Number(v.fold_lit_number(i)),
        Expr::Variable(i) => Expr::Variable(v.fold_lit_variable(i)),
        Expr::ExprOp(i) => Expr::ExprOp(v.fold_expr_op(i)),
    }
}
pub fn fold_not<V>(v: &mut V, i: Not) -> Not
where
    V: Fold + ?Sized,
{
    Not {
        expr: Box::new(v.fold_logical_expr(*i.expr)),
    }
}
pub fn fold_logical_op<V>(v: &mut V, i: LogicalOp) -> LogicalOp
where
    V: Fold + ?Sized,
{
    LogicalOp {
        op: v.fold_logical_opcode(i.op),
        left: Box::new(v.fold_logical_expr(*i.left)),
        right: Box::new(v.fold_logical_expr(*i.right)),
    }
}
pub fn fold_logical_expr_op<V>(v: &mut V, i: LogicalExprOp) -> LogicalExprOp
where
    V: Fold + ?Sized,
{
    LogicalExprOp {
        op: v.fold_logical_expr_opcode(i.op),
        left: Box::new(v.fold_expr(*i.left)),
        right: Box::new(v.fold_expr(*i.right)),
    }
}
pub fn fold_logical_opcode<V>(_v: &mut V, i: LogicalOpcode) -> LogicalOpcode
where
    V: Fold + ?Sized,
{
    i.clone()
}
pub fn fold_logical_expr_opcode<V>(_v: &mut V, i: LogicalExprOpcode) -> LogicalExprOpcode
where
    V: Fold + ?Sized,
{
    i.clone()
}
pub fn fold_logical_expr<V>(v: &mut V, i: LogicalExpr) -> LogicalExpr
where
    V: Fold + ?Sized,
{
    match i {
        LogicalExpr::Not(i) => LogicalExpr::Not(v.fold_not(i)),
        LogicalExpr::LogicalOp(i) => LogicalExpr::LogicalOp(v.fold_logical_op(i)),
        LogicalExpr::LogicalExprOp(i) => LogicalExpr::LogicalExprOp(v.fold_logical_expr_op(i)),
    }
}
pub fn fold_skip<V>(_v: &mut V, i: Skip) -> Skip
where
    V: Fold + ?Sized,
{
    Skip { ..i }
}
pub fn fold_diverge<V>(_v: &mut V, i: Diverge) -> Diverge
where
    V: Fold + ?Sized,
{
    Diverge { ..i }
}
pub fn fold_tick<V>(v: &mut V, i: Tick) -> Tick
where
    V: Fold + ?Sized,
{
    Tick {
        value: v.fold_lit_number(i.value),
    }
}
pub fn fold_assignment<V>(v: &mut V, i: Assignment) -> Assignment
where
    V: Fold + ?Sized,
{
    Assignment {
        name: v.fold_lit_variable(i.name),
        expr: Box::new(v.fold_expr(*i.expr)),
    }
}
pub fn fold_random_assignment<V>(v: &mut V, i: RandomAssignment) -> RandomAssignment
where
    V: Fold + ?Sized,
{
    RandomAssignment {
        name: v.fold_lit_variable(i.name),
        distribution: v.fold_prob_dist(i.distribution),
    }
}
pub fn fold_sequence<V>(v: &mut V, i: Sequence) -> Sequence
where
    V: Fold + ?Sized,
{
    Sequence {
        left: Box::new(v.fold_command(*i.left)),
        right: Box::new(v.fold_command(*i.right)),
    }
}
pub fn fold_nondeterministic_choice<V>(v: &mut V, i: NondetChoice) -> NondetChoice
where
    V: Fold + ?Sized,
{
    NondetChoice {
        left: Box::new(v.fold_command(*i.left)),
        right: Box::new(v.fold_command(*i.right)),
    }
}
pub fn fold_probabilistic_choice<V>(v: &mut V, i: ProbChoice) -> ProbChoice
where
    V: Fold + ?Sized,
{
    ProbChoice {
        left: Box::new(v.fold_command(*i.left)),
        prob: v.fold_lit_prob(i.prob),
        right: Box::new(v.fold_command(*i.right)),
    }
}
pub fn fold_if<V>(v: &mut V, i: If) -> If
where
    V: Fold + ?Sized,
{
    If {
        condition: Box::new(v.fold_logical_expr(*i.condition)),
        command: Box::new(v.fold_command(*i.command)),
    }
}
pub fn fold_if_else<V>(v: &mut V, i: IfElse) -> IfElse
where
    V: Fold + ?Sized,
{
    IfElse {
        condition: Box::new(v.fold_logical_expr(*i.condition)),
        left: Box::new(v.fold_command(*i.left)),
        right: Box::new(v.fold_command(*i.right)),
    }
}
pub fn fold_while<V>(v: &mut V, i: While) -> While
where
    V: Fold + ?Sized,
{
    While {
        condition: Box::new(v.fold_logical_expr(*i.condition)),
        command: Box::new(v.fold_command(*i.command)),
    }
}
pub fn fold_command<V>(v: &mut V, i: Command) -> Command
where
    V: Fold + ?Sized,
{
    match i {
        Command::Skip(i) => Command::Skip(v.fold_skip(i)),
        Command::Diverge(i) => Command::Diverge(v.fold_diverge(i)),
        Command::Tick(i) => Command::Tick(v.fold_tick(i)),
        Command::Assignment(i) => Command::Assignment(v.fold_assignment(i)),
        Command::RandomAssignment(i) => Command::RandomAssignment(v.fold_random_assignment(i)),
        Command::Sequence(i) => Command::Sequence(v.fold_sequence(i)),
        Command::NondetChoice(i) => Command::NondetChoice(v.fold_nondeterministic_choice(i)),
        Command::ProbChoice(i) => Command::ProbChoice(v.fold_probabilistic_choice(i)),
        Command::If(i) => Command::If(v.fold_if(i)),
        Command::IfElse(i) => Command::IfElse(v.fold_if_else(i)),
        Command::While(i) => Command::While(v.fold_while(i)),
    }
}

pub fn fold_lit_prob<V>(v: &mut V, i: LitProb) -> LitProb
where
    V: Fold + ?Sized,
{
    LitProb {
        value: v.fold_lit_number(i.value),
    }
}

pub fn fold_prob_dist<V>(v: &mut V, i: ProbDist) -> ProbDist
where
    V: Fold + ?Sized,
{
    match i {
        ProbDist::Normal(i) => ProbDist::Normal(v.fold_prob_dist_normal(i)),
        ProbDist::Uniform(i) => ProbDist::Uniform(v.fold_prob_dist_uniform(i)),
        ProbDist::LogNormal(i) => ProbDist::LogNormal(v.fold_prob_dist_log_normal(i)),
        ProbDist::Exponential(i) => ProbDist::Exponential(v.fold_prob_dist_exponential(i)),
    }
}

pub fn fold_prob_dist_normal<V>(v: &mut V, i: ProbDistNormal) -> ProbDistNormal
where
    V: Fold + ?Sized,
{
    ProbDistNormal {
        mean: v.fold_lit_number(i.mean),
        std_dev: v.fold_lit_number(i.std_dev),
    }
}
pub fn fold_prob_dist_uniform<V>(v: &mut V, i: ProbDistUniform) -> ProbDistUniform
where
    V: Fold + ?Sized,
{
    ProbDistUniform {
        min: v.fold_lit_number(i.min),
        max: v.fold_lit_number(i.max),
    }
}
pub fn fold_prob_dist_log_normal<V>(v: &mut V, i: ProbDistLogNormal) -> ProbDistLogNormal
where
    V: Fold + ?Sized,
{
    ProbDistLogNormal {
        mean: v.fold_lit_number(i.mean),
        std_dev: v.fold_lit_number(i.std_dev),
    }
}
pub fn fold_prob_dist_exponential<V>(v: &mut V, i: ProbDistExponential) -> ProbDistExponential
where
    V: Fold + ?Sized,
{
    ProbDistExponential {
        lambda: v.fold_lit_number(i.lambda),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar;

    #[test]
    fn test_fold() {
        let parser = grammar::commandParser::new();
        let expr = parser.parse("if (8 > 2) {skip}").unwrap();

        struct Transformer;
        impl Fold for Transformer {
            fn fold_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
                if let LogicalExprOpcode::GreaterThan = i.op {
                    return LogicalExprOp {
                        left: Box::new(self.fold_expr(*i.right)),
                        op: self.fold_logical_expr_opcode(LogicalExprOpcode::LessThan),
                        right: Box::new(self.fold_expr(*i.left)),
                    };
                }

                fold_logical_expr_op(self, i)
            }
        }

        let folded = Transformer.fold_command(*expr);
        assert_eq!("if (2 < 8) {skip}", format!("{:?}", folded));
    }
}
