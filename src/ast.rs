use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Number(i32),
    Variable(String),
    ExprOp(Box<Expr>, ExprOpcode, Box<Expr>),
}

pub enum ExprOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Monus,
    Mod,
}

pub enum LogicalExpr {
    Not(Box<LogicalExpr>),
    LogicalOp(Box<LogicalExpr>, LogicalOpcode, Box<LogicalExpr>),
    LogicalExprOp(Box<Expr>, LogicalExprOpcode, Box<Expr>),
}

pub enum LogicalOpcode {
    And,
    Or,
}

pub enum LogicalExprOpcode {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
}

pub enum Command {
    Skip,
    Diverge,
    Tick(i32),
    Assignment(String, Box<Expr>),
    RandomAssignment(String, ProbabilityDistribution),
    Sequence(Box<Command>, Box<Command>),
    NondeterministicChoice(Box<Command>, Box<Command>),
    ProbabilisticChoice(Box<Command>, Probability, Box<Command>),
    IfElse(Box<LogicalExpr>, Box<Command>, Box<Command>),
    While(Box<LogicalExpr>, Box<Command>),
}

pub enum Probability {
    Probability(f32),
}

pub enum ProbabilityDistribution {
    Normal(i32, i32),
    Uniform(i32),
    LogNormal(i32),
    Exponential(i32),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Number(n) => write!(fmt, "{:?}", n),
            Variable(s) => write!(fmt, "{:?}", s),
            ExprOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}
impl Debug for LogicalExpr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalExpr::*;
        match &*self {
            Not(b) => write!(fmt, "!{:?}", b),
            LogicalOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            LogicalExprOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
        }
    }
}

impl Debug for ExprOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ExprOpcode::*;
        match *self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Monus => write!(fmt, ":-"),
            Mod => write!(fmt, "%"),
        }
    }
}

impl Debug for LogicalOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalOpcode::*;
        match *self {
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
        }
    }
}

impl Debug for LogicalExprOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalExprOpcode::*;
        match *self {
            Equal => write!(fmt, "=="),
            NotEqual => write!(fmt, "!="),
            LessThan => write!(fmt, "<"),
            LessThanOrEq => write!(fmt, "<="),
            GreaterThan => write!(fmt, ">"),
            GreaterThanOrEq => write!(fmt, ">="),
        }
    }
}
