use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
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
    Tick(i32),
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
    Normal(i32, i32),
    Uniform(i32),
    LogNormal(i32, i32),
    Exponential(i32),
}

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match &*self {
//             Number(n) => write!(fmt, "{:?}", n),
//             Variable(s) => write!(fmt, "{:?}", s),
//             ExprOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//         }
//     }
// }
// impl Debug for LogicalExpr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::LogicalExpr::*;
//         match &*self {
//             Not(b) => write!(fmt, "!{:?}", b),
//             LogicalOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//             LogicalExprOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//         }
//     }
// }

// impl Debug for ExprOpcode {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::ExprOpcode::*;
//         match *self {
//             Add => write!(fmt, "+"),
//             Sub => write!(fmt, "-"),
//             Mul => write!(fmt, "*"),
//             Div => write!(fmt, "/"),
//             Monus => write!(fmt, ":-"),
//             Mod => write!(fmt, "%"),
//         }
//     }
// }

// impl Debug for LogicalOpcode {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::LogicalOpcode::*;
//         match *self {
//             And => write!(fmt, "&&"),
//             Or => write!(fmt, "||"),
//         }
//     }
// }

// impl Debug for LogicalExprOpcode {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::LogicalExprOpcode::*;
//         match *self {
//             Equal => write!(fmt, "=="),
//             NotEqual => write!(fmt, "!="),
//             LessThan => write!(fmt, "<"),
//             LessThanOrEq => write!(fmt, "<="),
//             GreaterThan => write!(fmt, ">"),
//             GreaterThanOrEq => write!(fmt, ">="),
//         }
//     }
// }

// impl Debug for Command {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Command::*;
//         match &*self {
//             Skip => write!(fmt, "skip"),
//             Diverge => write!(fmt, "diverge"),
//             Tick(n) => write!(fmt, "tick({:?})", n),
//             Assignment(var, val) => write!(fmt, "{:?} := {:?}", var, val),
//             RandomAssignment(var, dist) => write!(fmt, "{:?} := {:?}", var, dist),
//             Sequence(l, r) => write!(fmt, "{:?}; {:?}", l, r),
//             NondeterministicChoice(l, r) => write!(fmt, "{{{:?}}} [] {{{:?}}}", l, r),
//             ProbabilisticChoice(l, p, r) => write!(fmt, "{{{:?}}} [{:?}] {{{:?}}}", l, p, r),
//             If(g, c) => write!(fmt, "if {:?} {{ {:?} }}", g, c),
//             IfElse(g, c, e) => write!(fmt, "if {:?} {{ {:?} }} else {{ {:?} }}", g, c, e),
//             While(g, c) => write!(fmt, "while {:?} {{ {:?} }}", g, c),
//         }
//     }
// }

// impl Debug for ProbabilityDistribution {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::ProbabilityDistribution::*;
//         match *self {
//             Normal(l, r) => write!(fmt, "normal({:?}, {:?})", l, r),
//             Uniform(l) => write!(fmt, "uniform({:?})", l),
//             LogNormal(l, r) => write!(fmt, "lognormal({:?}, {:?})", l, r),
//             Exponential(l) => write!(fmt, "exponential({:?})", l),
//         }
//     }
// }

// impl Debug for Probability {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         match *self {
//             Probability::Probability(n) => write!(fmt, "{:?}", n),
//         }
//     }
// }
