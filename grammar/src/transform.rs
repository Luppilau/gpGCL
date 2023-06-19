use crate::ast::*;
#[allow(dead_code)]

pub trait Transform {
    fn transform_lit_number(&mut self, i: LitNumber) -> LitNumber {
        transform_lit_number(self, i)
    }
    fn transform_lit_variable(&mut self, i: LitVariable) -> LitVariable {
        transform_lit_variable(self, i)
    }
    fn transform_expr_op(&mut self, i: ExprOp) -> ExprOp {
        transform_expr_op(self, i)
    }
    fn transform_expr_opcode(&mut self, i: ExprOpcode) -> ExprOpcode {
        transform_expr_opcode(self, i)
    }
    fn transform_expr(&mut self, i: Expr) -> Expr {
        transform_expr(self, i)
    }
    fn transform_not(&mut self, i: Not) -> Not {
        transform_not(self, i)
    }
    fn transform_logical_op(&mut self, i: LogicalOp) -> LogicalOp {
        transform_logical_op(self, i)
    }
    fn transform_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
        transform_logical_expr_op(self, i)
    }
    fn transform_logical_opcode(&mut self, i: LogicalOpcode) -> LogicalOpcode {
        transform_logical_opcode(self, i)
    }
    fn transform_logical_expr_opcode(&mut self, i: LogicalExprOpcode) -> LogicalExprOpcode {
        transform_logical_expr_opcode(self, i)
    }
    fn transform_logical_expr(&mut self, i: LogicalExpr) -> LogicalExpr {
        transform_logical_expr(self, i)
    }
    fn transform_skip(&mut self, i: Skip) -> Skip {
        transform_skip(self, i)
    }
    fn transform_diverge(&mut self, i: Diverge) -> Diverge {
        transform_diverge(self, i)
    }
    fn transform_tick(&mut self, i: Tick) -> Tick {
        transform_tick(self, i)
    }
    fn transform_assignment(&mut self, i: Assignment) -> Assignment {
        transform_assignment(self, i)
    }
    fn transform_random_assignment(&mut self, i: RandomAssignment) -> RandomAssignment {
        transform_random_assignment(self, i)
    }
    fn transform_sequence(&mut self, i: Sequence) -> Sequence {
        transform_sequence(self, i)
    }
    fn transform_nondeterministic_choice(&mut self, i: NondetChoice) -> NondetChoice {
        transform_nondeterministic_choice(self, i)
    }
    fn transform_probabilistic_choice(&mut self, i: ProbChoice) -> ProbChoice {
        transform_probabilistic_choice(self, i)
    }
    fn transform_if(&mut self, i: If) -> If {
        transform_if(self, i)
    }
    fn transform_if_else(&mut self, i: IfElse) -> IfElse {
        transform_if_else(self, i)
    }
    fn transform_while(&mut self, i: While) -> While {
        transform_while(self, i)
    }
    fn transform_command(&mut self, i: Command) -> Command {
        transform_command(self, i)
    }

    fn transform_lit_prob(&mut self, i: LitProb) -> LitProb {
        transform_lit_prob(self, i)
    }
    fn transform_prob_dist(&mut self, i: ProbDist) -> ProbDist {
        transform_prob_dist(self, i)
    }
    fn transform_prob_dist_normal(&mut self, i: ProbDistNormal) -> ProbDistNormal {
        transform_prob_dist_normal(self, i)
    }
    fn transform_prob_dist_uniform(&mut self, i: ProbDistUniform) -> ProbDistUniform {
        transform_prob_dist_uniform(self, i)
    }
    fn transform_prob_dist_log_normal(&mut self, i: ProbDistLogNormal) -> ProbDistLogNormal {
        transform_prob_dist_log_normal(self, i)
    }
    fn transform_prob_dist_exponential(&mut self, i: ProbDistExponential) -> ProbDistExponential {
        transform_prob_dist_exponential(self, i)
    }
}

pub fn transform_lit_number<V>(_v: &mut V, i: LitNumber) -> LitNumber
where
    V: Transform + ?Sized,
{
    LitNumber { ..i }
}
pub fn transform_lit_variable<V>(_v: &mut V, i: LitVariable) -> LitVariable
where
    V: Transform + ?Sized,
{
    LitVariable { ..i }
}
pub fn transform_expr_op<V>(v: &mut V, i: ExprOp) -> ExprOp
where
    V: Transform + ?Sized,
{
    ExprOp {
        op: v.transform_expr_opcode(i.op),
        left: Box::new(v.transform_expr(*i.left)),
        right: Box::new(v.transform_expr(*i.right)),
    }
}
pub fn transform_expr_opcode<V>(_v: &mut V, i: ExprOpcode) -> ExprOpcode
where
    V: Transform + ?Sized,
{
    i.clone()
}
pub fn transform_expr<V>(v: &mut V, i: Expr) -> Expr
where
    V: Transform + ?Sized,
{
    match i {
        Expr::Number(i) => Expr::Number(v.transform_lit_number(i)),
        Expr::Variable(i) => Expr::Variable(v.transform_lit_variable(i)),
        Expr::ExprOp(i) => Expr::ExprOp(v.transform_expr_op(i)),
    }
}
pub fn transform_not<V>(v: &mut V, i: Not) -> Not
where
    V: Transform + ?Sized,
{
    Not {
        expr: Box::new(v.transform_logical_expr(*i.expr)),
    }
}
pub fn transform_logical_op<V>(v: &mut V, i: LogicalOp) -> LogicalOp
where
    V: Transform + ?Sized,
{
    LogicalOp {
        op: v.transform_logical_opcode(i.op),
        left: Box::new(v.transform_logical_expr(*i.left)),
        right: Box::new(v.transform_logical_expr(*i.right)),
    }
}
pub fn transform_logical_expr_op<V>(v: &mut V, i: LogicalExprOp) -> LogicalExprOp
where
    V: Transform + ?Sized,
{
    LogicalExprOp {
        op: v.transform_logical_expr_opcode(i.op),
        left: Box::new(v.transform_expr(*i.left)),
        right: Box::new(v.transform_expr(*i.right)),
    }
}
pub fn transform_logical_opcode<V>(_v: &mut V, i: LogicalOpcode) -> LogicalOpcode
where
    V: Transform + ?Sized,
{
    i.clone()
}
pub fn transform_logical_expr_opcode<V>(_v: &mut V, i: LogicalExprOpcode) -> LogicalExprOpcode
where
    V: Transform + ?Sized,
{
    i.clone()
}
pub fn transform_logical_expr<V>(v: &mut V, i: LogicalExpr) -> LogicalExpr
where
    V: Transform + ?Sized,
{
    match i {
        LogicalExpr::Not(i) => LogicalExpr::Not(v.transform_not(i)),
        LogicalExpr::LogicalOp(i) => LogicalExpr::LogicalOp(v.transform_logical_op(i)),
        LogicalExpr::LogicalExprOp(i) => LogicalExpr::LogicalExprOp(v.transform_logical_expr_op(i)),
    }
}
pub fn transform_skip<V>(_v: &mut V, i: Skip) -> Skip
where
    V: Transform + ?Sized,
{
    Skip { ..i }
}
pub fn transform_diverge<V>(_v: &mut V, i: Diverge) -> Diverge
where
    V: Transform + ?Sized,
{
    Diverge { ..i }
}
pub fn transform_tick<V>(v: &mut V, i: Tick) -> Tick
where
    V: Transform + ?Sized,
{
    Tick {
        value: v.transform_lit_number(i.value),
    }
}
pub fn transform_assignment<V>(v: &mut V, i: Assignment) -> Assignment
where
    V: Transform + ?Sized,
{
    Assignment {
        name: v.transform_lit_variable(i.name),
        expr: Box::new(v.transform_expr(*i.expr)),
    }
}
pub fn transform_random_assignment<V>(v: &mut V, i: RandomAssignment) -> RandomAssignment
where
    V: Transform + ?Sized,
{
    RandomAssignment {
        name: v.transform_lit_variable(i.name),
        distribution: v.transform_prob_dist(i.distribution),
    }
}
pub fn transform_sequence<V>(v: &mut V, i: Sequence) -> Sequence
where
    V: Transform + ?Sized,
{
    Sequence {
        left: Box::new(v.transform_command(*i.left)),
        right: Box::new(v.transform_command(*i.right)),
    }
}
pub fn transform_nondeterministic_choice<V>(v: &mut V, i: NondetChoice) -> NondetChoice
where
    V: Transform + ?Sized,
{
    NondetChoice {
        left: Box::new(v.transform_command(*i.left)),
        right: Box::new(v.transform_command(*i.right)),
    }
}
pub fn transform_probabilistic_choice<V>(v: &mut V, i: ProbChoice) -> ProbChoice
where
    V: Transform + ?Sized,
{
    ProbChoice {
        left: Box::new(v.transform_command(*i.left)),
        prob: v.transform_lit_prob(i.prob),
        right: Box::new(v.transform_command(*i.right)),
    }
}
pub fn transform_if<V>(v: &mut V, i: If) -> If
where
    V: Transform + ?Sized,
{
    If {
        condition: Box::new(v.transform_logical_expr(*i.condition)),
        command: Box::new(v.transform_command(*i.command)),
    }
}
pub fn transform_if_else<V>(v: &mut V, i: IfElse) -> IfElse
where
    V: Transform + ?Sized,
{
    IfElse {
        condition: Box::new(v.transform_logical_expr(*i.condition)),
        left: Box::new(v.transform_command(*i.left)),
        right: Box::new(v.transform_command(*i.right)),
    }
}
pub fn transform_while<V>(v: &mut V, i: While) -> While
where
    V: Transform + ?Sized,
{
    While {
        condition: Box::new(v.transform_logical_expr(*i.condition)),
        command: Box::new(v.transform_command(*i.command)),
    }
}
pub fn transform_command<V>(v: &mut V, i: Command) -> Command
where
    V: Transform + ?Sized,
{
    match i {
        Command::Skip(i) => Command::Skip(v.transform_skip(i)),
        Command::Diverge(i) => Command::Diverge(v.transform_diverge(i)),
        Command::Tick(i) => Command::Tick(v.transform_tick(i)),
        Command::Assignment(i) => Command::Assignment(v.transform_assignment(i)),
        Command::RandomAssignment(i) => Command::RandomAssignment(v.transform_random_assignment(i)),
        Command::Sequence(i) => Command::Sequence(v.transform_sequence(i)),
        Command::NondetChoice(i) => Command::NondetChoice(v.transform_nondeterministic_choice(i)),
        Command::ProbChoice(i) => Command::ProbChoice(v.transform_probabilistic_choice(i)),
        Command::If(i) => Command::If(v.transform_if(i)),
        Command::IfElse(i) => Command::IfElse(v.transform_if_else(i)),
        Command::While(i) => Command::While(v.transform_while(i)),
    }
}

pub fn transform_lit_prob<V>(v: &mut V, i: LitProb) -> LitProb
where
    V: Transform + ?Sized,
{
    LitProb {
        value: v.transform_lit_number(i.value),
    }
}

pub fn transform_prob_dist<V>(v: &mut V, i: ProbDist) -> ProbDist
where
    V: Transform + ?Sized,
{
    match i {
        ProbDist::Normal(i) => ProbDist::Normal(v.transform_prob_dist_normal(i)),
        ProbDist::Uniform(i) => ProbDist::Uniform(v.transform_prob_dist_uniform(i)),
        ProbDist::LogNormal(i) => ProbDist::LogNormal(v.transform_prob_dist_log_normal(i)),
        ProbDist::Exponential(i) => ProbDist::Exponential(v.transform_prob_dist_exponential(i)),
    }
}

pub fn transform_prob_dist_normal<V>(v: &mut V, i: ProbDistNormal) -> ProbDistNormal
where
    V: Transform + ?Sized,
{
    ProbDistNormal {
        mean: v.transform_lit_number(i.mean),
        std_dev: v.transform_lit_number(i.std_dev),
    }
}
pub fn transform_prob_dist_uniform<V>(v: &mut V, i: ProbDistUniform) -> ProbDistUniform
where
    V: Transform + ?Sized,
{
    ProbDistUniform {
        min: v.transform_lit_number(i.min),
        max: v.transform_lit_number(i.max),
    }
}
pub fn transform_prob_dist_log_normal<V>(v: &mut V, i: ProbDistLogNormal) -> ProbDistLogNormal
where
    V: Transform + ?Sized,
{
    ProbDistLogNormal {
        mean: v.transform_lit_number(i.mean),
        std_dev: v.transform_lit_number(i.std_dev),
    }
}
pub fn transform_prob_dist_exponential<V>(v: &mut V, i: ProbDistExponential) -> ProbDistExponential
where
    V: Transform + ?Sized,
{
    ProbDistExponential {
        lambda: v.transform_lit_number(i.lambda),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar;

    #[test]
    fn test_transform() {
        let parser = grammar::commandParser::new();
        let expr = parser.parse("if (8 > 2) {skip}").unwrap();

        struct Transformer;
        impl Transform for Transformer {
            fn transform_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
                if let LogicalExprOpcode::GreaterThan = i.op {
                    return LogicalExprOp {
                        left: Box::new(self.transform_expr(*i.right)),
                        op: self.transform_logical_expr_opcode(LogicalExprOpcode::LessThan),
                        right: Box::new(self.transform_expr(*i.left)),
                    };
                }

                transform_logical_expr_op(self, i)
            }
        }

        let transformed = Transformer.transform_command(*expr);
        assert_eq!("if (2 < 8) {skip}", format!("{:?}", transformed));
    }
}
