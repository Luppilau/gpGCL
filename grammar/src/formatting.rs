use crate::ast;

use ast::*;

trait Transform<T> {
    fn transform(&self) -> &T;
}

impl Transform<Expr> for Expr {
    fn transform(&self) -> &Expr {
        self
    }
}
