use crate::ast::*;
#[allow(dead_code)]

/// # Transpile
/// Use this trait to transpile an AST into a string.
///
/// To override the default implementation of a function, simply implement the function for your
/// transpiler. For example, to override the default implementation of `transpile_assignment`, you
/// would implement `transpile_assignment` for your transpiler.
///
/// NOTE - It is important to use the transpile functions on nested AST nodes. For example, if you
/// are implementing `transpile_expr`, you should use `transpile_expr` on the `Expr` in the
/// `ExprOp` that you are transpiling. This is to continue the recursive transpilation process.
///
///
/// ## Example
/// ```
/// use grammar::ast::*;
/// use grammar::transpile::{self, Transpile};
///
/// struct Transpiler;
/// impl Transpile for Transpiler {
///     fn transpile_assignment(&mut self, i: Assignment) -> String {
///         format!(
///             "{} :=== {}",
///             transpile::transpile_lit_variable(self, i.name),
///             transpile::transpile_expr(self, *i.expr)
///         )
///     }
/// }
/// ```
///
/// This transpiler will override the default transpilation for assignment, which is `x := 1` into a string of the form `x :=== 1`.
///
/// ### Usage of this transpiler:
/// ```ignore
/// let ast = Transpiler.transpile_command(ast);
/// println!("{}", ast);
/// ```
pub trait Transpile {
    fn transpile_lit_number(&mut self, i: LitNumber) -> String {
        transpile_lit_number(self, i)
    }
    fn transpile_lit_variable(&mut self, i: LitVariable) -> String {
        transpile_lit_variable(self, i)
    }
    fn transpile_expr_op(&mut self, i: ExprOp) -> String {
        transpile_expr_op(self, i)
    }
    fn transpile_expr_opcode(&mut self, i: ExprOpcode) -> String {
        transpile_expr_opcode(self, i)
    }
    fn transpile_expr(&mut self, i: Expr) -> String {
        transpile_expr(self, i)
    }
    fn transpile_not(&mut self, i: Not) -> String {
        transpile_not(self, i)
    }
    fn transpile_logical_op(&mut self, i: LogicalOp) -> String {
        transpile_logical_op(self, i)
    }
    fn transpile_logical_expr_op(&mut self, i: LogicalExprOp) -> String {
        transpile_logical_expr_op(self, i)
    }
    fn transpile_logical_opcode(&mut self, i: LogicalOpcode) -> String {
        transpile_logical_opcode(self, i)
    }
    fn transpile_logical_expr_opcode(&mut self, i: LogicalExprOpcode) -> String {
        transpile_logical_expr_opcode(self, i)
    }
    fn transpile_logical_expr(&mut self, i: LogicalExpr) -> String {
        transpile_logical_expr(self, i)
    }
    fn transpile_skip(&mut self, i: Skip) -> String {
        transpile_skip(self, i)
    }
    fn transpile_diverge(&mut self, i: Diverge) -> String {
        transpile_diverge(self, i)
    }
    fn transpile_tick(&mut self, i: Tick) -> String {
        transpile_tick(self, i)
    }
    fn transpile_assignment(&mut self, i: Assignment) -> String {
        transpile_assignment(self, i)
    }
    fn transpile_random_assignment(&mut self, i: RandomAssignment) -> String {
        transpile_random_assignment(self, i)
    }
    fn transpile_sequence(&mut self, i: Sequence) -> String {
        transpile_sequence(self, i)
    }
    fn transpile_nondeterministic_choice(&mut self, i: NondetChoice) -> String {
        transpile_nondeterministic_choice(self, i)
    }
    fn transpile_probabilistic_choice(&mut self, i: ProbChoice) -> String {
        transpile_probabilistic_choice(self, i)
    }
    fn transpile_if(&mut self, i: If) -> String {
        transpile_if(self, i)
    }
    fn transpile_if_else(&mut self, i: IfElse) -> String {
        transpile_if_else(self, i)
    }
    fn transpile_while(&mut self, i: While) -> String {
        transpile_while(self, i)
    }
    fn transpile_command(&mut self, i: Command) -> String {
        transpile_command(self, i)
    }

    fn transpile_lit_prob(&mut self, i: LitProb) -> String {
        transpile_lit_prob(self, i)
    }
    fn transpile_prob_dist(&mut self, i: ProbDist) -> String {
        transpile_prob_dist(self, i)
    }
    fn transpile_prob_dist_normal(&mut self, i: ProbDistNormal) -> String {
        transpile_prob_dist_normal(self, i)
    }
    fn transpile_prob_dist_uniform(&mut self, i: ProbDistUniform) -> String {
        transpile_prob_dist_uniform(self, i)
    }
    fn transpile_prob_dist_log_normal(&mut self, i: ProbDistLogNormal) -> String {
        transpile_prob_dist_log_normal(self, i)
    }
    fn transpile_prob_dist_exponential(&mut self, i: ProbDistExponential) -> String {
        transpile_prob_dist_exponential(self, i)
    }
}

pub fn transpile_lit_number<V>(_v: &mut V, i: LitNumber) -> String
where
    V: Transpile + ?Sized,
{
    i.value.to_string()
}
pub fn transpile_lit_variable<V>(_v: &mut V, i: LitVariable) -> String
where
    V: Transpile + ?Sized,
{
    i.name
}
pub fn transpile_expr_op<V>(v: &mut V, i: ExprOp) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{} {} {}",
        v.transpile_expr(*i.left),
        v.transpile_expr_opcode(i.op),
        v.transpile_expr(*i.right)
    )
}
pub fn transpile_expr_opcode<V>(_v: &mut V, i: ExprOpcode) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        ExprOpcode::Add => "+".to_string(),
        ExprOpcode::Sub => "-".to_string(),
        ExprOpcode::Mul => "*".to_string(),
        ExprOpcode::Div => "/".to_string(),
        ExprOpcode::Monus => ":-".to_string(),
        ExprOpcode::Mod => "%".to_string(),
    }
}
pub fn transpile_expr<V>(v: &mut V, i: Expr) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        Expr::Number(i) => v.transpile_lit_number(i),
        Expr::Variable(i) => v.transpile_lit_variable(i),
        Expr::ExprOp(i) => v.transpile_expr_op(i),
    }
}
pub fn transpile_not<V>(v: &mut V, i: Not) -> String
where
    V: Transpile + ?Sized,
{
    format!("!{}", v.transpile_logical_expr(*i.expr))
}
pub fn transpile_logical_op<V>(v: &mut V, i: LogicalOp) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{} {} {}",
        v.transpile_logical_expr(*i.left),
        v.transpile_logical_opcode(i.op),
        v.transpile_logical_expr(*i.right)
    )
}
pub fn transpile_logical_expr_op<V>(v: &mut V, i: LogicalExprOp) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{} {} {}",
        v.transpile_expr(*i.left),
        v.transpile_logical_expr_opcode(i.op),
        v.transpile_expr(*i.right)
    )
}
pub fn transpile_logical_opcode<V>(_v: &mut V, i: LogicalOpcode) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        LogicalOpcode::And => "&&".to_string(),
        LogicalOpcode::Or => "||".to_string(),
    }
}
pub fn transpile_logical_expr_opcode<V>(_v: &mut V, i: LogicalExprOpcode) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        LogicalExprOpcode::Equal => "==".to_string(),
        LogicalExprOpcode::NotEqual => "!=".to_string(),
        LogicalExprOpcode::LessThan => "<".to_string(),
        LogicalExprOpcode::GreaterThan => ">".to_string(),
        LogicalExprOpcode::LessThanOrEq => "<=".to_string(),
        LogicalExprOpcode::GreaterThanOrEq => ">=".to_string(),
    }
}
pub fn transpile_logical_expr<V>(v: &mut V, i: LogicalExpr) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        LogicalExpr::Not(i) => v.transpile_not(i),
        LogicalExpr::LogicalOp(i) => v.transpile_logical_op(i),
        LogicalExpr::LogicalExprOp(i) => v.transpile_logical_expr_op(i),
    }
}
pub fn transpile_skip<V>(_v: &mut V, _i: Skip) -> String
where
    V: Transpile + ?Sized,
{
    "skip".to_string()
}
pub fn transpile_diverge<V>(_v: &mut V, _i: Diverge) -> String
where
    V: Transpile + ?Sized,
{
    "diverge".to_string()
}
pub fn transpile_tick<V>(v: &mut V, i: Tick) -> String
where
    V: Transpile + ?Sized,
{
    format!("tick({})", v.transpile_lit_number(i.value))
}
pub fn transpile_assignment<V>(v: &mut V, i: Assignment) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{} := {}",
        v.transpile_lit_variable(i.name),
        v.transpile_expr(*i.expr)
    )
}
pub fn transpile_random_assignment<V>(v: &mut V, i: RandomAssignment) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{} := {}",
        v.transpile_lit_variable(i.name),
        v.transpile_prob_dist(i.distribution)
    )
}
pub fn transpile_sequence<V>(v: &mut V, i: Sequence) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{}; {}",
        v.transpile_command(*i.left),
        v.transpile_command(*i.right)
    )
}
pub fn transpile_nondeterministic_choice<V>(v: &mut V, i: NondetChoice) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{{{}}} [] {{{}}}",
        v.transpile_command(*i.left),
        v.transpile_command(*i.right)
    )
}
pub fn transpile_probabilistic_choice<V>(v: &mut V, i: ProbChoice) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "{{{}}} [{}] {{{}}}",
        v.transpile_command(*i.left),
        v.transpile_lit_prob(i.prob),
        v.transpile_command(*i.right)
    )
}
pub fn transpile_if<V>(v: &mut V, i: If) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "if ({}) {{{}}}",
        v.transpile_logical_expr(*i.condition),
        v.transpile_command(*i.command)
    )
}
pub fn transpile_if_else<V>(v: &mut V, i: IfElse) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "if ({}) {{{}}} else {{{}}}",
        v.transpile_logical_expr(*i.condition),
        v.transpile_command(*i.left),
        v.transpile_command(*i.right)
    )
}
pub fn transpile_while<V>(v: &mut V, i: While) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "while ({}) {{{}}}",
        v.transpile_logical_expr(*i.condition),
        v.transpile_command(*i.command)
    )
}
pub fn transpile_command<V>(v: &mut V, i: Command) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        Command::Skip(i) => v.transpile_skip(i),
        Command::Diverge(i) => v.transpile_diverge(i),
        Command::Tick(i) => v.transpile_tick(i),
        Command::Assignment(i) => v.transpile_assignment(i),
        Command::RandomAssignment(i) => v.transpile_random_assignment(i),
        Command::Sequence(i) => v.transpile_sequence(i),
        Command::NondetChoice(i) => v.transpile_nondeterministic_choice(i),
        Command::ProbChoice(i) => v.transpile_probabilistic_choice(i),
        Command::If(i) => v.transpile_if(i),
        Command::IfElse(i) => v.transpile_if_else(i),
        Command::While(i) => v.transpile_while(i),
    }
}

pub fn transpile_lit_prob<V>(v: &mut V, i: LitProb) -> String
where
    V: Transpile + ?Sized,
{
    format!("{}", v.transpile_lit_number(i.value))
}

pub fn transpile_prob_dist<V>(v: &mut V, i: ProbDist) -> String
where
    V: Transpile + ?Sized,
{
    match i {
        ProbDist::Normal(i) => v.transpile_prob_dist_normal(i),
        ProbDist::Uniform(i) => v.transpile_prob_dist_uniform(i),
        ProbDist::LogNormal(i) => v.transpile_prob_dist_log_normal(i),
        ProbDist::Exponential(i) => v.transpile_prob_dist_exponential(i),
    }
}

pub fn transpile_prob_dist_normal<V>(v: &mut V, i: ProbDistNormal) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "normal({}, {})",
        v.transpile_lit_number(i.mean),
        v.transpile_lit_number(i.std_dev)
    )
}
pub fn transpile_prob_dist_uniform<V>(v: &mut V, i: ProbDistUniform) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "uniform({}, {})",
        v.transpile_lit_number(i.min),
        v.transpile_lit_number(i.max)
    )
}
pub fn transpile_prob_dist_log_normal<V>(v: &mut V, i: ProbDistLogNormal) -> String
where
    V: Transpile + ?Sized,
{
    format!(
        "lognormal({}, {})",
        v.transpile_lit_number(i.mean),
        v.transpile_lit_number(i.std_dev)
    )
}
pub fn transpile_prob_dist_exponential<V>(v: &mut V, i: ProbDistExponential) -> String
where
    V: Transpile + ?Sized,
{
    format!("exponential({})", v.transpile_lit_number(i.lambda))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar;

    #[test]
    fn test_transpile() {
        let parser = grammar::commandParser::new();
        let expr = parser.parse("if (8 > 2) {skip}").unwrap();

        struct DefaultTranspiler;
        impl Transpile for DefaultTranspiler {}

        let transpiled = DefaultTranspiler.transpile_command(*expr);
        assert_eq!("if (8 > 2) {skip}", format!("{}", transpiled));
    }

    #[test]
    fn test_custom_transpile() {
        let parser = grammar::commandParser::new();
        let expr = parser.parse("if (8 > 2) {skip}").unwrap();

        struct CustomTranspiler;
        impl Transpile for CustomTranspiler {
            fn transpile_if(&mut self, i: If) -> String {
                format!(
                    "if {} {{{}}}",
                    self.transpile_logical_expr(*i.condition),
                    self.transpile_command(*i.command)
                )
            }

            fn transpile_skip(&mut self, _: Skip) -> String {
                "dontdoanything".to_string()
            }
        }

        let transpiled = CustomTranspiler.transpile_command(*expr);
        assert_eq!("if 8 > 2 {dontdoanything}", format!("{}", transpiled));
    }
}
