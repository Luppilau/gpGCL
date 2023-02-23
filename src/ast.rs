pub enum Expr {
    Literal(i32),
    Variable(String),
    Times(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
}

pub enum LogicalExpr {
    LessThan(Box<Expr>, Box<Expr>),
    Not(Box<LogicalExpr>),
    And(Box<LogicalExpr>, Box<LogicalExpr>),
}

pub enum Command {
    Skip,
    Assignment(String, Box<Expr>),
    Sequence(Box<Command>, Box<Command>),
    ProbabilisticChoice(Box<Command>, Probability, Box<Command>),
    Conditional(Box<LogicalExpr>, Box<Command>, Probability, Box<Command>),
}

pub enum Probability {
    Probability(f32),
}
