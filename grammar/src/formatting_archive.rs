// use std::fmt::{Error, Formatter};

// use crate::ast;
// use crate::grammar;

// pub struct CustomFormatter {
//     expr_formatter: ExprFormatter,
//     expr_opcode_formatter: ExprOpcodeFormatter,
//     logical_expr_formatter: LogicalExprFormatter,
//     logical_opcode_formatter: LogicalOpcodeFormatter,
//     logical_expr_opcode_formatter: LogicalExprOpcodeFormatter,
//     command_formatter: CommandFormatter,
//     probability_formatter: ProbabilityFormatter,
//     probability_distribution_formatter: ProbabilityDistributionFormatter,
// }

// struct ExprFormatter {
//     number_format: Box<dyn Fn(&i32) -> String>,
//     variable_format: Box<dyn Fn(&String) -> String>,
//     expr_op_format: Box<dyn Fn(&Expr, &ExprOpcode, &Expr) -> String>,
// }

// impl ExprFormatter {
//     fn new() -> ExprFormatter {
//         ExprFormatter {
//             number_format: Box::new(|n| format!("{}", n)),
//             variable_format: Box::new(|s| format!("{}", s)),
//             expr_op_format: Box::new(|l, op, r| format!("{} {} {}", l, op, r)),
//         }
//     }
// }

// struct ExprOpcodeFormatter {
//     add_format: Box<dyn Fn() -> String>,
//     sub_format: Box<dyn Fn() -> String>,
//     mul_format: Box<dyn Fn() -> String>,
//     div_format: Box<dyn Fn() -> String>,
//     monus_format: Box<dyn Fn() -> String>,
//     mod_format: Box<dyn Fn() -> String>,
// }

// impl ExprOpcodeFormatter {
//     fn new() -> ExprOpcodeFormatter {
//         ExprOpcodeFormatter {
//             add_format: Box::new(|| "+".to_string()),
//             sub_format: Box::new(|| "-".to_string()),
//             mul_format: Box::new(|| "*".to_string()),
//             div_format: Box::new(|| "/".to_string()),
//             monus_format: Box::new(|| ":-".to_string()),
//             mod_format: Box::new(|| "%".to_string()),
//         }
//     }
// }

// struct LogicalExprFormatter {
//     not_format: Box<dyn Fn(&LogicalExpr) -> String>,
//     logical_op_format: Box<dyn Fn(&LogicalExpr, &LogicalOpcode, &LogicalExpr) -> String>,
//     logical_expr_op_format: Box<dyn Fn(&Expr, &LogicalExprOpcode, &Expr) -> String>,
// }

// impl LogicalExprFormatter {
//     fn new() -> LogicalExprFormatter {
//         LogicalExprFormatter {
//             not_format: Box::new(|e| format!("!{}", e)),
//             logical_op_format: Box::new(|l, op, r| format!("{} {} {}", l, op, r)),
//             logical_expr_op_format: Box::new(|l, op, r| format!("{} {} {}", l, op, r)),
//         }
//     }
// }

// struct LogicalOpcodeFormatter {
//     and_format: Box<dyn Fn() -> String>,
//     or_format: Box<dyn Fn() -> String>,
// }

// impl LogicalOpcodeFormatter {
//     fn new() -> LogicalOpcodeFormatter {
//         LogicalOpcodeFormatter {
//             and_format: Box::new(|| "&".to_string()),
//             or_format: Box::new(|| "|".to_string()),
//         }
//     }
// }

// struct LogicalExprOpcodeFormatter {
//     equal_format: Box<dyn Fn() -> String>,
//     not_equal_format: Box<dyn Fn() -> String>,
//     less_than_format: Box<dyn Fn() -> String>,
//     less_than_or_eq_format: Box<dyn Fn() -> String>,
//     greater_than_format: Box<dyn Fn() -> String>,
//     greater_than_or_eq_format: Box<dyn Fn() -> String>,
// }

// impl LogicalExprOpcodeFormatter {
//     fn new() -> LogicalExprOpcodeFormatter {
//         LogicalExprOpcodeFormatter {
//             equal_format: Box::new(|| "==".to_string()),
//             not_equal_format: Box::new(|| "!=".to_string()),
//             less_than_format: Box::new(|| "<".to_string()),
//             less_than_or_eq_format: Box::new(|| "<=".to_string()),
//             greater_than_format: Box::new(|| ">".to_string()),
//             greater_than_or_eq_format: Box::new(|| ">=".to_string()),
//         }
//     }
// }

// struct CommandFormatter {
//     skip_format: Box<dyn Fn() -> String>,
//     diverge_format: Box<dyn Fn() -> String>,
//     tick_format: Box<dyn Fn(&i32) -> String>,
//     assignment_format: Box<dyn Fn(&String, &Expr) -> String>,
//     random_assignment_format: Box<dyn Fn(&String, &ProbabilityDistribution) -> String>,
//     sequence_format: Box<dyn Fn(&Command, &Command) -> String>,
//     nondeterministic_choice_format: Box<dyn Fn(&Command, &Command) -> String>,
//     probabilistic_choice_format: Box<dyn Fn(&Command, &Probability, &Command) -> String>,
//     if_format: Box<dyn Fn(&LogicalExpr, &Command) -> String>,
//     if_else_format: Box<dyn Fn(&LogicalExpr, &Command, &Command) -> String>,
//     while_format: Box<dyn Fn(&LogicalExpr, &Command) -> String>,
// }

// impl CommandFormatter {
//     fn new() -> CommandFormatter {
//         CommandFormatter {
//             skip_format: Box::new(|| "skip".to_string()),
//             diverge_format: Box::new(|| "diverge".to_string()),
//             tick_format: Box::new(|n| format!("tick({})", n)),
//             assignment_format: Box::new(|v, e| format!("{} := {}", v, e)),
//             random_assignment_format: Box::new(|v, d| format!("{} := {}", v, d)),
//             sequence_format: Box::new(|l, r| format!("{}; {}", l, r)),
//             nondeterministic_choice_format: Box::new(|l, r| format!("{} + {}", l, r)),
//             probabilistic_choice_format: Box::new(|l, p, r| format!("{} + {} : {}", l, p, r)),
//             if_format: Box::new(|e, c| format!("if {} then {}", e, c)),
//             if_else_format: Box::new(|e, l, r| format!("if {} then {} else {}", e, l, r)),
//             while_format: Box::new(|e, c| format!("while {} do {}", e, c)),
//         }
//     }
// }

// struct ProbabilityFormatter {
//     probability_format: Box<dyn Fn(&f64) -> String>,
// }

// impl ProbabilityFormatter {
//     fn new() -> ProbabilityFormatter {
//         ProbabilityFormatter {
//             probability_format: Box::new(|p| format!("{}", p)),
//         }
//     }
// }

// struct ProbabilityDistributionFormatter {
//     normal_format: Box<dyn Fn(&Expr, &Expr) -> String>,
//     uniform_format: Box<dyn Fn(&Expr) -> String>,
//     log_normal_format: Box<dyn Fn(&Expr, &Expr) -> String>,
//     exponential_format: Box<dyn Fn(&Expr) -> String>,
// }

// impl ProbabilityDistributionFormatter {
//     fn new() -> ProbabilityDistributionFormatter {
//         ProbabilityDistributionFormatter {
//             normal_format: Box::new(|m, s| format!("normal({}, {})", m, s)),
//             uniform_format: Box::new(|s| format!("uniform({})", s)),
//             log_normal_format: Box::new(|m, s| format!("log_normal({}, {})", m, s)),
//             exponential_format: Box::new(|l| format!("exponential({})", l)),
//         }
//     }
// }
