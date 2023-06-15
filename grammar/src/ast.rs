use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct LitNumber {
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LitVariable {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprOp {
    pub left: Box<Expr>,
    pub op: ExprOpcode,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(LitNumber),
    Variable(LitVariable),
    ExprOp(ExprOp),
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
    Not(Not),
    LogicalOp(LogicalOp),
    LogicalExprOp(LogicalExprOp),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Not {
    pub expr: Box<LogicalExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalOp {
    pub left: Box<LogicalExpr>,
    pub op: LogicalOpcode,
    pub right: Box<LogicalExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalExprOp {
    pub left: Box<Expr>,
    pub op: LogicalExprOpcode,
    pub right: Box<Expr>,
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
pub struct Skip {}

#[derive(Debug, Clone, PartialEq)]
pub struct Diverge {}

#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub value: LitNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub name: LitVariable,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RandomAssignment {
    pub name: LitVariable,
    pub distribution: ProbabilityDistribution,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NondeterministicChoice {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticChoice {
    pub left: Box<Command>,
    pub probability: Probability,
    pub right: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub condition: Box<LogicalExpr>,
    pub command: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElse {
    pub condition: Box<LogicalExpr>,
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Box<LogicalExpr>,
    pub command: Box<Command>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Skip(Skip),
    Diverge(Diverge),
    Tick(Tick),
    Assignment(Assignment),
    RandomAssignment(RandomAssignment),
    Sequence(Sequence),
    NondeterministicChoice(NondeterministicChoice),
    ProbabilisticChoice(ProbabilisticChoice),
    If(If),
    IfElse(IfElse),
    While(While),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Probability {
    Probability(ProbabilityLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityLiteral {
    pub value: LitNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProbabilityDistribution {
    Normal(ProbabilityDistributionNormal),
    Uniform(ProbabilityDistributionUniform),
    LogNormal(ProbabilityDistributionLogNormal),
    Exponential(ProbabilityDistributionExponential),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistributionNormal {
    pub mean: LitNumber,
    pub std_dev: LitNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistributionUniform {
    pub min: LitNumber,
    pub max: LitNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistributionLogNormal {
    pub mean: LitNumber,
    pub std_dev: LitNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistributionExponential {
    pub lambda: LitNumber,
}
