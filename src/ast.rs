#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    ExprOp(Box<Expr>, ExprOpcode, Box<Expr>),
}

#[derive(Debug)]
pub enum ExprOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Monus,
    Mod,
}

#[derive(Debug)]
pub enum LogicalExpr {
    Not(Box<LogicalExpr>),
    LogicalOp(Box<LogicalExpr>, LogicalOpcode, Box<LogicalExpr>),
    LogicalExprOp(Box<Expr>, LogicalExprOpcode, Box<Expr>),
}

#[derive(Debug)]
pub enum LogicalOpcode {
    And,
    Or,
}

#[derive(Debug)]
pub enum LogicalExprOpcode {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
}

#[derive(Debug)]
pub enum Command {
    Skip,
    Diverge,
    Tick(i32),
    Assignment(String, Box<Expr>),
    RandomAssignment(String, ProbabilityDistribution),
    Sequence(Box<Command>, Box<Command>),
    ProbabilisticChoice(Box<Command>, Probability, Box<Command>),
    NondeterministicChoice(Box<Command>, Box<Command>),
    Conditional(Box<LogicalExpr>, Box<Command>, Box<Command>),
    While(Box<LogicalExpr>, Box<Command>),
}

#[derive(Debug)]
pub enum Probability {
    Probability(f32),
}

#[derive(Debug)]
pub enum ProbabilityDistribution {
    Normal(i32, i32),
    Uniform(i32),
    LogNormal(i32),
    Exponential(i32),
}
