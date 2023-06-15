use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f32),
    Variable(String),
    ExprOp(Box<Expr>, ExprOpcode, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Monus,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalExpr {
    Not(Box<LogicalExpr>),
    LogicalOp(Box<LogicalExpr>, LogicalOpcode, Box<LogicalExpr>),
    LogicalExprOp(Box<Expr>, LogicalExprOpcode, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOpcode {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalExprOpcode {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Skip,
    Diverge,
    Tick(f32),
    Assignment(String, Box<Expr>),
    RandomAssignment(String, ProbabilityDistribution),
    Sequence(Box<Command>, Box<Command>),
    NondeterministicChoice(Box<Command>, Box<Command>),
    ProbabilisticChoice(Box<Command>, Probability, Box<Command>),
    If(Box<LogicalExpr>, Box<Command>),
    IfElse(Box<LogicalExpr>, Box<Command>, Box<Command>),
    While(Box<LogicalExpr>, Box<Command>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Probability {
    Probability(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProbabilityDistribution {
    Normal(f32, f32),
    Uniform(f32, f32),
    LogNormal(f32, f32),
    Exponential(f32),
}

////////////////////////////////////

////////////////////////////////////

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Number(n) => write!(fmt, "{}", n),
            Variable(s) => write!(fmt, "{s}"),
            ExprOp(ref l, op, ref r) => write!(fmt, "{l} {op} {r}"),
        }
    }
}

impl Display for LogicalExpr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalExpr::*;
        match &*self {
            Not(b) => write!(fmt, "!{b}"),
            LogicalOp(ref l, op, ref r) => write!(fmt, "{l} {op} {r}"),
            LogicalExprOp(ref l, op, ref r) => write!(fmt, "{l} {op} {r}"),
        }
    }
}

impl Display for ExprOpcode {
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

impl Display for LogicalOpcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalOpcode::*;
        match *self {
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
        }
    }
}

impl Display for LogicalExprOpcode {
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

impl Display for Command {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Command::*;
        match &*self {
            Skip => write!(fmt, "skip"),
            Diverge => write!(fmt, "diverge"),
            Tick(n) => write!(fmt, "tick({n})"),
            Assignment(var, val) => write!(fmt, "{var} := {val}"),
            RandomAssignment(var, dist) => write!(fmt, "{var} := {dist}"),
            Sequence(l, r) => write!(fmt, "{l}; {r}"),
            NondeterministicChoice(l, r) => write!(fmt, "{{{l}}} [] {{{r}}}"),
            ProbabilisticChoice(l, p, r) => write!(fmt, "{{{l}}} [{p}] {{{r}}}"),
            If(g, c) => write!(fmt, "if {g} {{ {c} }}"),
            IfElse(g, c, e) => write!(fmt, "if {g} {{ {c} }} else {{ {e} }}"),
            While(g, c) => write!(fmt, "while {g} {{ {c} }}"),
        }
    }
}

impl Display for ProbabilityDistribution {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ProbabilityDistribution::*;
        match *self {
            Normal(l, r) => write!(fmt, "normal({l}, {r})"),
            Uniform(l, r) => write!(fmt, "uniform({l}, {r})"),
            LogNormal(l, r) => write!(fmt, "lognormal({l}, {r})"),
            Exponential(l) => write!(fmt, "exponential({l})"),
        }
    }
}

impl Display for Probability {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Probability::*;
        match *self {
            Probability(p) => write!(fmt, "{p}"),
        }
    }
}
