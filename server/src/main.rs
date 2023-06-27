#[macro_use]
extern crate rocket;
extern crate lalrpop_util;

mod cors;
mod handler;

use std::{fs::File, io::Write, process::Command as CommandProcess};

use grammar::ast::*;
use grammar::transform::{self, Transform};
use grammar::transpile::Transpile;
use handler::{CustomHandler, Handler, ParseError, RequestHandler};
use serde::{Deserialize, Serialize};

impl RequestHandler for CustomHandler {
    // TODO: Implement custom transpilation here
    fn transpile(&self, ast: Command) -> String {
        struct Transpiler;
        impl grammar::transpile::Transpile for Transpiler {
            fn transpile_logical_expr_opcode(&mut self, i: LogicalExprOpcode) -> String {
                match i {
                    LogicalExprOpcode::Equal => "=".to_string(),
                    LogicalExprOpcode::LessThan => "<".to_string(),
                    LogicalExprOpcode::LessThanOrEq => "<=".to_string(),
                    _ => unreachable!("Invalid logical expression opcode"),
                }
            }

            fn transpile_logical_opcode(&mut self, i: LogicalOpcode) -> String {
                match i {
                    LogicalOpcode::And => "&".to_string(),
                    LogicalOpcode::Or => "||".to_string(),
                }
            }

            fn transpile_prob_dist_uniform(&mut self, i: ProbDistUniform) -> String {
                format!(
                    "unif({}, {})",
                    grammar::transpile::transpile_lit_number(self, i.min),
                    grammar::transpile::transpile_lit_number(self, i.max)
                )
            }
        }

        Transpiler.transpile_command(ast)
    }

    // TODO: Implement custom transformation here
    fn transform(&self, ast: Command) -> Command {
        struct Transformer;
        impl transform::Transform for Transformer {
            fn transform_logical_expr_op(&mut self, i: LogicalExprOp) -> LogicalExprOp {
                match i.op {
                    LogicalExprOpcode::GreaterThan => LogicalExprOp {
                        op: transform::transform_logical_expr_opcode(
                            self,
                            LogicalExprOpcode::LessThan,
                        ),
                        left: Box::new(transform::transform_expr(self, *i.right)),
                        right: Box::new(transform::transform_expr(self, *i.left)),
                    },
                    LogicalExprOpcode::GreaterThanOrEq => LogicalExprOp {
                        op: transform::transform_logical_expr_opcode(
                            self,
                            LogicalExprOpcode::LessThanOrEq,
                        ),
                        left: Box::new(transform::transform_expr(self, *i.right)),
                        right: Box::new(transform::transform_expr(self, *i.left)),
                    },

                    _ => transform::transform_logical_expr_op(self, i),
                }
            }
        }

        Transformer.transform_command(ast)
    }

    fn validate(&self, ast: &Command) -> Result<(), String> {
        struct Visitor {
            errors: Vec<String>,
        }

        impl grammar::visit::Visit for Visitor {
            fn visit_diverge(&mut self, _i: &Diverge) {
                self.errors
                    .push("Diverge operation not supported".to_string());
            }
            fn visit_nondeterministic_choice(&mut self, _i: &NondetChoice) {
                self.errors
                    .push("Nondeterministic choice not supported".to_string());
            }
            fn visit_logical_expr_opcode(&mut self, i: &LogicalExprOpcode) {
                match i {
                    LogicalExprOpcode::NotEqual => self
                        .errors
                        .push("Not equal operator not supported".to_string()),
                    _ => (),
                }
            }
            fn visit_prob_dist_normal(&mut self, i: &ProbDistNormal) {
                self.errors
                    .push("Normal distribution not supported".to_string());
            }

            fn visit_prob_dist_log_normal(&mut self, i: &ProbDistLogNormal) {
                self.errors
                    .push("Lognormal distribution not supported".to_string());
            }
            fn visit_prob_dist_exponential(&mut self, i: &ProbDistExponential) {
                self.errors
                    .push("Exponential distribution not supported".to_string());
            }
        }
        let mut checker = Visitor { errors: vec![] };

        grammar::visit::visit_command(&mut checker, ast);
        if checker.errors.len() > 0 {
            return Err(checker.errors.join("\n"));
        } else {
            return Ok(());
        };
    }
}

#[derive(Serialize)]
struct ValidationResponse {
    errors: Vec<ParseError>,
}

#[post("/validate", data = "<input>")]
fn validate(input: String) -> String {
    let result = &CustomHandler.validate_input(&input);

    let response = match result {
        Ok(_) => ValidationResponse { errors: vec![] },
        Err(errors) => ValidationResponse {
            errors: vec![errors.clone()],
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[derive(Deserialize, Debug)]
struct ExecutionRequest {
    program: String,
    args: String,
}

#[derive(Serialize)]
struct ExecutionResponse {
    result: String,
    errors: Vec<String>,
}

#[post("/execute", data = "<input>")]
fn execute(input: String) -> String {
    let request: ExecutionRequest = serde_json::from_str(&input).unwrap();

    // Parse input and return error if necessary
    let parsed_source: Result<Command, ParseError> = CustomHandler.parse_input(&request.program);
    if let Err(e) = parsed_source {
        let response = ExecutionResponse {
            result: "".to_string(),
            errors: vec!["Parsing error".to_string(), e.message().to_string()],
        };

        return serde_json::to_string(&response).unwrap();
    }

    // Transform input using CustomHandler
    let transformed_source = CustomHandler.transform(parsed_source.unwrap());

    let mut assignment_checker = AssignmentVisitor {
        variable_declarations: vec![],
        errors: vec![],
    };

    grammar::visit::visit_command(&mut assignment_checker, &transformed_source);

    // Transpile input
    let transpiled_source = &CustomHandler.transpile(transformed_source);

    let transpiled_source = format!(
        "{}{}",
        assignment_checker
            .variable_declarations
            .iter()
            .fold(String::new(), |acc, item| {
                acc + &format!("nat {};\n", item)
            }),
        transpiled_source
    );

    // TODO: Call tool with transpiled

    let mut file = File::create("temp.imp.pgcl").unwrap();
    file.write_all(transpiled_source.as_bytes()).unwrap();

    println!("Transpiled source: {}", transpiled_source);

    let mut process = CommandProcess::new("sh");
    process.current_dir("cegispro2");
    process.arg("-c").arg(format!(
        "poetry run python3 -m cegispro2.cmd ../temp.imp.pgcl {}",
        request.args
    ));

    let output = process.output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap().to_string();

    let respose = ExecutionResponse {
        result: std::str::from_utf8(&output.stdout).unwrap().to_string(),
        errors: match stderr.len() {
            0 => vec![],
            _ => vec![stderr],
        },
    };

    // Respond with result
    serde_json::to_string(&respose).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![validate, execute])
}

struct AssignmentVisitor {
    variable_declarations: Vec<String>,
    errors: Vec<String>,
}

impl grammar::visit::Visit for AssignmentVisitor {
    fn visit_assignment(&mut self, i: &Assignment) {
        if !self.variable_declarations.contains(&i.name.name) {
            self.variable_declarations.push(i.name.name.clone())
        }
    }

    fn visit_random_assignment(&mut self, i: &RandomAssignment) {
        if !self.variable_declarations.contains(&i.name.name) {
            self.variable_declarations.push(i.name.name.clone())
        }
    }
}
