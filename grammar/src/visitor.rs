// use crate::cst::*;

// trait Accept<T> {
//     fn accept(&self, v: &dyn Visitor<Output = T>) -> T;
// }

// impl<T> Accept<T> for Expr {
//     fn accept(&self, v: &dyn Visitor<Output = T>) -> T {
//         v.visit_expr(&self)
//     }
// }

// pub trait Visitor {
//     type Output;

//     fn visit_expr(&self, expr: &Expr) -> Self::Output;
// }
