#[macro_use]
extern crate rocket;
extern crate lalrpop_util;

mod cors;
mod handler;

use grammar::ast::*;
use grammar::visit::Visit;
use handler::{ParseError, RequestHandler};
use serde::Serialize;

pub struct CustomHandler;
impl RequestHandler for CustomHandler {
    fn visit(&self, _ast: &grammar::ast::Command) -> Result<(), String> {
        struct SupportChecker {
            errors: Vec<String>,
        }

        impl grammar::visit::Visit for SupportChecker {
            fn visit_diverge(&mut self, _i: &Diverge) {
                self.errors
                    .push("Diverge operation not supported".to_string());
            }

            fn visit_skip(&mut self, _i: &Skip) {
                self.errors.push("Skip operation not supported".to_string());
            }
        }

        let mut checker = SupportChecker { errors: vec![] };

        checker.visit_command(_ast);
        if !checker.errors.is_empty() {
            return Err(checker.errors.join("\n"));
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct Response {
    result: String,
    errors: Vec<ParseError>,
}

#[post("/validate", data = "<input>")]
fn validate(input: String) -> String {
    let result = &CustomHandler.validate(&input);

    let response = match result {
        Ok(_) => Response {
            result: "".to_string(),
            errors: vec![],
        },
        Err(errors) => Response {
            result: "".to_string(),
            errors: vec![errors.clone()],
        },
    };

    serde_json::to_string(&response).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![validate])
}
