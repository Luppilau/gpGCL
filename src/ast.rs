#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Variable(String),
    ExprOp(Box<Expr>, ExprOpcode, Box<Expr>),
}

#[derive(Debug)]
pub enum ExprOpcode {
    Add,
    Mul,
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
}

#[derive(Debug)]
pub enum LogicalExprOpcode {
    LessThan,
}

#[derive(Debug)]
pub enum Command {
    Skip,
    Assignment(String, Box<Expr>),
    Sequence(Box<Command>, Box<Command>),
    ProbabilisticChoice(Box<Command>, Probability, Box<Command>),
    Conditional(Box<LogicalExpr>, Box<Command>, Probability, Box<Command>),
}

#[derive(Debug)]
pub enum Probability {
    Probability(f32),
}
