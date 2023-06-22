#[macro_use]
extern crate rocket;

mod cors;
mod validate;

use grammar::ast::*;
use grammar::visit::Visit;
use validate::*;

pub struct RequestHandler;
impl validate::Responder for RequestHandler {
    fn validate(&self, _ast: &grammar::ast::Command) -> Result<(), String> {
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

#[post("/handle", data = "<input>")]
fn handle(input: String) -> String {
    RequestHandler.handle(&input)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![handle])
}
