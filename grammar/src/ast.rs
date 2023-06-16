use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, PartialEq)]
pub struct LitNumber {
    pub value: f32,
}

#[derive(Clone, PartialEq)]
pub struct LitVariable {
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct ExprOp {
    pub left: Box<Expr>,
    pub op: ExprOpcode,
    pub right: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    Number(LitNumber),
    Variable(LitVariable),
    ExprOp(ExprOp),
}

#[derive(Clone, PartialEq)]
pub enum ExprOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Monus,
    Mod,
}

#[derive(Clone, PartialEq)]
pub enum LogicalExpr {
    Not(Not),
    LogicalOp(LogicalOp),
    LogicalExprOp(LogicalExprOp),
}

#[derive(Clone, PartialEq)]
pub struct Not {
    pub expr: Box<LogicalExpr>,
}

#[derive(Clone, PartialEq)]
pub struct LogicalOp {
    pub left: Box<LogicalExpr>,
    pub op: LogicalOpcode,
    pub right: Box<LogicalExpr>,
}

#[derive(Clone, PartialEq)]
pub struct LogicalExprOp {
    pub left: Box<Expr>,
    pub op: LogicalExprOpcode,
    pub right: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub enum LogicalOpcode {
    And,
    Or,
}

#[derive(Clone, PartialEq)]
pub enum LogicalExprOpcode {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
}

#[derive(Clone, PartialEq)]
pub struct Skip {}

#[derive(Clone, PartialEq)]
pub struct Diverge {}

#[derive(Clone, PartialEq)]
pub struct Tick {
    pub value: LitNumber,
}

#[derive(Clone, PartialEq)]
pub struct Assignment {
    pub name: LitVariable,
    pub expr: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct RandomAssignment {
    pub name: LitVariable,
    pub distribution: ProbDist,
}

#[derive(Clone, PartialEq)]
pub struct Sequence {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub struct NondetChoice {
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub struct ProbChoice {
    pub left: Box<Command>,
    pub prob: LitProb,
    pub right: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub struct If {
    pub condition: Box<LogicalExpr>,
    pub command: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub struct IfElse {
    pub condition: Box<LogicalExpr>,
    pub left: Box<Command>,
    pub right: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub struct While {
    pub condition: Box<LogicalExpr>,
    pub command: Box<Command>,
}

#[derive(Clone, PartialEq)]
pub enum Command {
    Skip(Skip),
    Diverge(Diverge),
    Tick(Tick),
    Assignment(Assignment),
    RandomAssignment(RandomAssignment),
    Sequence(Sequence),
    NondetChoice(NondetChoice),
    ProbChoice(ProbChoice),
    If(If),
    IfElse(IfElse),
    While(While),
}

#[derive(Clone, PartialEq)]
pub struct LitProb {
    pub value: LitNumber,
}

#[derive(Clone, PartialEq)]
pub enum ProbDist {
    Normal(ProbDistNormal),
    Uniform(ProbDistUniform),
    LogNormal(ProbDistLogNormal),
    Exponential(ProbDistExponential),
}

#[derive(Clone, PartialEq)]
pub struct ProbDistNormal {
    pub mean: LitNumber,
    pub std_dev: LitNumber,
}

#[derive(Clone, PartialEq)]
pub struct ProbDistUniform {
    pub min: LitNumber,
    pub max: LitNumber,
}

#[derive(Clone, PartialEq)]
pub struct ProbDistLogNormal {
    pub mean: LitNumber,
    pub std_dev: LitNumber,
}

#[derive(Clone, PartialEq)]
pub struct ProbDistExponential {
    pub lambda: LitNumber,
}

//////////////////////////

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Number(n) => n.fmt(fmt),
            Variable(n) => n.fmt(fmt),
            ExprOp(n) => n.fmt(fmt),
        }
    }
}

impl Debug for LitNumber {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.value)
    }
}

impl Debug for LitVariable {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}

impl Debug for ExprOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "{left:?} {op:?} {right:?}",
            left = self.left,
            op = self.op,
            right = self.right
        )
    }
}

impl Debug for LogicalExpr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::LogicalExpr::*;
        match &*self {
            Not(n) => n.fmt(fmt),
            LogicalOp(n) => n.fmt(fmt),
            LogicalExprOp(n) => n.fmt(fmt),
        }
    }
}

impl Debug for Not {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "!{:?}", self.expr)
    }
}

impl Debug for LogicalOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "{left:?} {op:?} {right:?}",
            left = self.left,
            op = self.op,
            right = self.right
        )
    }
}

impl Debug for LogicalExprOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "{left:?} {op:?} {right:?}",
            left = self.left,
            op = self.op,
            right = self.right
        )
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

impl Debug for Command {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Command::*;
        match &*self {
            Skip(n) => n.fmt(fmt),
            Diverge(n) => n.fmt(fmt),
            Tick(n) => n.fmt(fmt),
            Assignment(n) => n.fmt(fmt),
            RandomAssignment(n) => n.fmt(fmt),
            Sequence(n) => n.fmt(fmt),
            NondetChoice(n) => n.fmt(fmt),
            ProbChoice(n) => n.fmt(fmt),
            If(n) => n.fmt(fmt),
            IfElse(n) => n.fmt(fmt),
            While(n) => n.fmt(fmt),
        }
    }
}

impl Debug for Skip {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "skip")
    }
}

impl Debug for Diverge {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "diverge")
    }
}

impl Debug for Tick {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "tick")
    }
}

impl Debug for Assignment {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?} := {:?}", self.name, self.expr)
    }
}

impl Debug for RandomAssignment {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?} := {:?}", self.name, self.distribution)
    }
}

impl Debug for Sequence {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}; {:?}", self.left, self.right)
    }
}

impl Debug for NondetChoice {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{{{:?}}} [] {{{:?}}}", self.left, self.right)
    }
}

impl Debug for ProbChoice {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "{{{:?}}} [{:?}] {{{:?}}}",
            self.left, self.prob, self.right
        )
    }
}

impl Debug for If {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "if ({:?}) {{{:?}}}", self.condition, self.command)
    }
}

impl Debug for IfElse {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "if ({:?}) {{{:?}}} else {{{:?}}}",
            self.condition, self.left, self.right
        )
    }
}

impl Debug for While {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "while ({:?}) {{{:?}}}", self.condition, self.command)
    }
}

impl Debug for ProbDist {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::ProbDist::*;
        match self {
            Normal(n) => n.fmt(fmt),
            Uniform(n) => n.fmt(fmt),
            LogNormal(n) => n.fmt(fmt),
            Exponential(n) => n.fmt(fmt),
        }
    }
}

impl Debug for ProbDistNormal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "normal({mean:?}, {std_dev:?})",
            mean = self.mean,
            std_dev = self.std_dev
        )
    }
}

impl Debug for ProbDistUniform {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "uniform({min:?}, {max:?})",
            min = self.min,
            max = self.max
        )
    }
}

impl Debug for ProbDistLogNormal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "lognormal({mean:?}, {std_dev:?})",
            mean = self.mean,
            std_dev = self.std_dev
        )
    }
}

impl Debug for ProbDistExponential {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "exponential({lambda:?})", lambda = self.lambda)
    }
}

impl Debug for LitProb {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar;

    #[test]
    fn logical() {
        let parser = grammar::logical_exprParser::new();

        assert!(parser.parse("!22 < 21").is_ok());
        assert!(parser.parse("1 < 1 && 1 < 1").is_ok());
        assert!(parser.parse("1 < 1 || 1 < 1").is_ok());
        assert!(parser.parse("22 < 21").is_ok());
        assert!(parser.parse("22 <= 21").is_ok());
        assert!(parser.parse("22 > 22").is_ok());
        assert!(parser.parse("22 >= 22").is_ok());
        assert!(parser.parse("22 == 22").is_ok());
        assert!(parser.parse("22 != 22").is_ok());

        assert!(parser.parse("22 !!= 22").is_err());
        assert!(parser.parse("22 < !22").is_err());
        assert!(parser.parse("22 < 22 < 22").is_err());
    }

    // #[test]

    // fn logical_associativity() {
    //     let parser = grammar::logical_exprParser::new();

    //     let expr = parser.parse("!1>2 && 1>2").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "Not(LogicalOp(LogicalExprOp(Number(1), GreaterThan, Number(2)), And, LogicalExprOp(Number(1), GreaterThan, Number(2))))"
    //     );

    //     let expr = parser.parse("(!1>2) && 1>2").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "LogicalOp(Not(LogicalExprOp(Number(1), GreaterThan, Number(2))), And, LogicalExprOp(Number(1), GreaterThan, Number(2)))"
    //     );

    //     let expr = parser.parse("!(1>2) && (1>2)").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "Not(LogicalOp(LogicalExprOp(Number(1), GreaterThan, Number(2)), And, LogicalExprOp(Number(1), GreaterThan, Number(2))))"
    //     );

    //     let expr = parser.parse("1>2 && 1>2 || 1>2").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "LogicalOp(LogicalOp(LogicalExprOp(Number(1), GreaterThan, Number(2)), And, LogicalExprOp(Number(1), GreaterThan, Number(2))), Or, LogicalExprOp(Number(1), GreaterThan, Number(2)))"
    //     );
    // }

    #[test]
    fn expression() {
        let parser = grammar::exprParser::new();

        assert!(parser.parse("22").is_ok());
        assert!(parser.parse("22.1").is_ok());
        assert!(parser.parse("variablename").is_ok());
        assert!(parser.parse("22 * 22").is_ok());
        assert!(parser.parse("22 / 22").is_ok());
        assert!(parser.parse("22 % 22").is_ok());
        assert!(parser.parse("22 + 22").is_ok());
        assert!(parser.parse("22 - 22").is_ok());
        assert!(parser.parse("22 :- 22").is_ok());
        assert!(parser.parse("a + a * a").is_ok());
        assert!(parser.parse("a * a + a").is_ok());
        assert!(parser.parse("(22)").is_ok());

        assert!(parser.parse("22 * + 22").is_err());
        assert!(parser.parse("a A").is_err());
        assert!(parser.parse("a1 + a").is_err());
    }

    // #[test]
    // fn expression_associativity() {
    //     let parser = grammar::exprParser::new();

    //     let expr = parser.parse("1 * 2 + 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "ExprOp(ExprOp(Number(1), Mul, Number(2)), Add, Number(3))"
    //     );

    //     let expr = parser.parse("1 / 2 - 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "ExprOp(ExprOp(Number(1), Div, Number(2)), Sub, Number(3))"
    //     );

    //     let expr = parser.parse("1 % 2 :- 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "ExprOp(ExprOp(Number(1), Mod, Number(2)), Monus, Number(3))"
    //     );

    //     let expr = parser.parse("1 * (2 + 3)").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "ExprOp(Number(1), Mul, ExprOp(Number(2), Add, Number(3)))"
    //     );
    // }
    #[test]
    fn command() {
        let parser = grammar::commandParser::new();

        assert!(parser.parse("skip").is_ok());
        assert!(parser.parse("diverge").is_ok());
        assert!(parser.parse("tick(1)").is_ok());
        assert!(parser.parse("tick(0.1)").is_ok());
        assert!(parser.parse("x:=1").is_ok());
        assert!(parser.parse("x := normal(1,2)").is_ok());
        assert!(parser.parse("x := uniform(1,2)").is_ok());
        assert!(parser.parse("x := lognormal(1,2)").is_ok());
        assert!(parser.parse("x := exponential(1)").is_ok());
        assert!(parser.parse("skip ; skip").is_ok());
        assert!(parser.parse("{skip} [] {skip}").is_ok());
        assert!(parser.parse("{skip} [0.1] {skip}").is_ok());
        assert!(parser.parse("if (a==0) { skip }").is_ok());
        assert!(parser.parse("if (a==0) { skip } else { skip }").is_ok());
        assert!(parser.parse("while (a > b) { skip }").is_ok());
        assert!(parser
            .parse(
                "
            if (a==0) { skip };
            if (a==0) { skip } else { skip }
            "
            )
            .is_ok());
        assert!(parser
            .parse(
                "if (x>=y) {
                {skip} [] {skip}
            } else {
                diverge
            }
            "
            )
            .is_ok());

        // let expr = parser
        //     .parse(
        //         "
        // if (0 == 0) {
        //     {skip; skip} [] {skip; skip}
        // };
        // {skip; skip} [0.2] {tick(1)};
        // x:=exponential(1);
        // while(a>b) {
        //     skip
        // }
        // ",
        //     )
        //     .unwrap();
        // assert_eq!(
        //     &format!("{:?}", expr),
        //     "Sequence(Sequence(Sequence(If(LogicalExprOp(Number(0), Equal, Number(0)), NondeterministicChoice(Sequence(Skip, Skip), Sequence(Skip, Skip))), ProbabilisticChoice(Sequence(Skip, Skip), Probability(0.2), Tick(1))), RandomAssignment(\"x\", Exponential(1))), While(LogicalExprOp(Variable(\"a\"), GreaterThan, Variable(\"b\")), Skip))"
        // );
    }

    // #[test]
    // fn command_associativity() {
    //     let parser = grammar::commandParser::new();

    //     let expr = parser.parse("skip; skip; skip").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "Sequence(Sequence(Skip, Skip), Skip)"
    //     );

    //     let expr = parser.parse("skip; skip; skip; skip").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "Sequence(Sequence(Sequence(Skip, Skip), Skip), Skip)"
    //     );
    // }
}
