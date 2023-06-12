use crate::ast;

use ast::*;

pub struct AstTransformer {
    expr_transformer: Box<dyn Fn(&mut ast::Expr) -> &mut ast::Expr>,
    expr_opcode_transformer: Box<dyn Fn(&mut ast::ExprOpcode) -> &mut ast::ExprOpcode>,
    logical_expr_transformer: Box<dyn Fn(&mut ast::LogicalExpr) -> &mut ast::LogicalExpr>,
    logical_opcode_transformer: Box<dyn Fn(&mut ast::LogicalOpcode) -> &mut ast::LogicalOpcode>,
    logical_expr_opcode_transformer:
        Box<dyn Fn(&mut ast::LogicalExprOpcode) -> &mut ast::LogicalExprOpcode>,
    command_transformer: Box<dyn Fn(&mut ast::Command) -> &mut ast::Command>,
    probability_transformer: Box<dyn Fn(&mut ast::Probability) -> &mut ast::Probability>,
    probability_distribution_transformer:
        Box<dyn Fn(&mut ast::ProbabilityDistribution) -> &mut ast::ProbabilityDistribution>,
}

impl AstTransformer {
    pub fn new() -> Self {
        AstTransformer {
            expr_transformer: Box::new(|x| x),
            expr_opcode_transformer: Box::new(|x| x),
            logical_expr_transformer: Box::new(|x| x),
            logical_opcode_transformer: Box::new(|x| x),
            logical_expr_opcode_transformer: Box::new(|x| x),
            command_transformer: Box::new(|x| x),
            probability_transformer: Box::new(|x| x),
            probability_distribution_transformer: Box::new(|x| x),
        }
    }

    pub fn set_expr_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::Expr) -> &mut ast::Expr + 'static,
    {
        self.expr_transformer = Box::new(f);
    }

    pub fn set_expr_opcode_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::ExprOpcode) -> &mut ast::ExprOpcode + 'static,
    {
        self.expr_opcode_transformer = Box::new(f);
    }

    pub fn set_logical_expr_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::LogicalExpr) -> &mut ast::LogicalExpr + 'static,
    {
        self.logical_expr_transformer = Box::new(f);
    }

    pub fn set_logical_opcode_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::LogicalOpcode) -> &mut ast::LogicalOpcode + 'static,
    {
        self.logical_opcode_transformer = Box::new(f);
    }

    pub fn set_logical_expr_opcode_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::LogicalExprOpcode) -> &mut ast::LogicalExprOpcode + 'static,
    {
        self.logical_expr_opcode_transformer = Box::new(f);
    }

    pub fn set_command_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::Command) -> &mut ast::Command + 'static,
    {
        self.command_transformer = Box::new(f);
    }

    pub fn set_probability_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::Probability) -> &mut ast::Probability + 'static,
    {
        self.probability_transformer = Box::new(f);
    }

    pub fn set_probability_distribution_transformer<F>(&mut self, f: F)
    where
        F: Fn(&mut ast::ProbabilityDistribution) -> &mut ast::ProbabilityDistribution + 'static,
    {
        self.probability_distribution_transformer = Box::new(f);
    }

    fn transform_expr<'a>(&'a self, expr: &'a mut ast::Expr) -> &'a mut ast::Expr {
        (self.expr_transformer)(expr)
    }
    fn transform_expr_opcode<'a>(
        &'a self,
        expr: &'a mut ast::ExprOpcode,
    ) -> &'a mut ast::ExprOpcode {
        (self.expr_opcode_transformer)(expr)
    }
    fn transform_logical_expr<'a>(
        &'a self,
        expr: &'a mut ast::LogicalExpr,
    ) -> &'a mut ast::LogicalExpr {
        (self.logical_expr_transformer)(expr)
    }
    fn transform_logical_opcode<'a>(
        &'a self,
        expr: &'a mut ast::LogicalOpcode,
    ) -> &'a mut ast::LogicalOpcode {
        (self.logical_opcode_transformer)(expr)
    }
    fn transform_logical_expr_opcode<'a>(
        &'a self,
        expr: &'a mut ast::LogicalExprOpcode,
    ) -> &'a mut ast::LogicalExprOpcode {
        (self.logical_expr_opcode_transformer)(expr)
    }
    fn transform_command<'a>(&'a self, expr: &'a mut ast::Command) -> &'a mut ast::Command {
        (self.command_transformer)(expr)
    }
    fn transform_probability<'a>(
        &'a self,
        expr: &'a mut ast::Probability,
    ) -> &'a mut ast::Probability {
        (self.probability_transformer)(expr)
    }
    fn transform_probability_distribution<'a>(
        &'a self,
        expr: &'a mut ast::ProbabilityDistribution,
    ) -> &'a mut ast::ProbabilityDistribution {
        (self.probability_distribution_transformer)(expr)
    }
}

pub trait Transformable {
    fn transform<'a>(&'a mut self, transformer: &'a AstTransformer) -> &'a mut Self;
}

impl Transformable for Expr {
    fn transform<'a>(&'a mut self, transformer: &'a AstTransformer) -> &'a mut Self {
        use self::Expr::*;
        println!("transforming here: {self}");
        let result = transformer.transform_expr(self);
        println!("transforming done: {result}\n");
        match result {
            Number(_) => {}
            Variable(_) => {}
            ExprOp(l, _, r) => {
                l.transform(transformer);
                r.transform(transformer);
            }
        }

        result
    }
}

impl Transformable for ExprOpcode {
    fn transform<'a>(&'a mut self, transformer: &'a AstTransformer) -> &'a mut Self {
        use self::ExprOpcode::*;
        println!("transforming here: {self}");
        let result = transformer.transform_expr_opcode(self);
        println!("transforming done: {result}\n");
        match result {
            Add => {}
            Sub => {}
            Mul => {}
            Div => {}
            Monus => {}
            Mod => {}
        }
        result
    }
}

impl Transformable for LogicalExpr {
    fn transform<'a>(&'a mut self, transformer: &'a AstTransformer) -> &'a mut Self {
        use self::LogicalExpr::*;
        println!("transforming here: {self}");
        let result = transformer.transform_logical_expr(self);
        println!("transforming done: {result}\n");
        match result {
            Not(_) => {}
            LogicalOp(_, _, _) => {}
            LogicalExprOp(l, _, r) => {
                l.transform(transformer);
                r.transform(transformer);
            }
        }
        result
    }
}
impl Transformable for LogicalOpcode {
    fn transform(&mut self, transformer: &AstTransformer) -> &mut Self {
        use self::LogicalOpcode::*;
        match self {
            And => {}
            Or => {}
        }
        self
    }
}

impl Transformable for Command {
    fn transform(&mut self, transformer: &AstTransformer) -> &mut Self {
        use self::Command::*;
        match self {
            Skip => {}
            Diverge => {}
            Tick(_) => {}
            Assignment(_, e) => {
                e.transform(transformer);
            }
            RandomAssignment(_, _) => {}
            Sequence(l, r) => {
                l.transform(transformer);
                r.transform(transformer);
            }
            NondeterministicChoice(l, r) => {
                l.transform(transformer);
                r.transform(transformer);
            }
            ProbabilisticChoice(l, _, r) => {
                l.transform(transformer);
                r.transform(transformer);
            }
            If(g, c) => {
                g.transform(transformer);
                c.transform(transformer);
            }
            IfElse(g, c, e) => {
                g.transform(transformer);
                c.transform(transformer);
                e.transform(transformer);
            }
            While(g, c) => {
                g.transform(transformer);
                c.transform(transformer);
            }
        }
        self
    }
}

impl Transformable for ProbabilityDistribution {
    fn transform(&mut self, transformer: &AstTransformer) -> &mut Self {
        use self::ProbabilityDistribution::*;
        match self {
            Normal(_, _) => {}
            Uniform(_, _) => {}
            LogNormal(_, _) => {}
            Exponential(_) => {}
        }
        self
    }
}

impl Transformable for Probability {
    fn transform(&mut self, transformer: &AstTransformer) -> &mut Self {
        use self::Probability::*;
        match self {
            Probability(_) => {}
        }
        self
    }
}
