#[macro_use]
extern crate rocket;
extern crate lalrpop_util;

mod cors;
mod handler;

use grammar::ast::*;
use handler::{ParseError, RequestHandler};
use serde::{Deserialize, Serialize};

pub struct CustomHandler;
impl RequestHandler for CustomHandler {
    // TODO: Implement custom transpilation here
    // fn transpile(&self, ast: Command) -> String {
    //     struct Transpiler;
    //     impl grammar::transpile::Transpile for Transpiler {}

    //     Transpiler.transpile_command(ast)
    // }

    // TODO: Implement custom transformation here
    // fn transform(&self, ast: Command) -> Command {
    //     struct Transformer;
    //     impl grammar::transform::Transform for Transformer {}

    //     Transformer.transform_command(ast)
    // }

    // TODO: Implement custom visitation here
    // fn visit(&self, ast: &Command) -> Result<(), String> {
    //     struct Visitor;
    //     impl Visit for Visitor {}

    //     Ok(Visitor.visit_command(ast))
    // }
}

#[derive(Serialize)]
struct ValidationResponse {
    result: String,
    errors: Vec<ParseError>,
}

#[post("/validate", data = "<input>")]
fn validate(input: String) -> String {
    let result = &CustomHandler.validate(&input);

    let response = match result {
        Ok(_) => ValidationResponse {
            result: "".to_string(),
            errors: vec![],
        },
        Err(errors) => ValidationResponse {
            result: "".to_string(),
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
    errors: Vec<ParseError>,
}

#[post("/execute", data = "<input>")]
fn execute(input: String) -> String {
    let request: ExecutionRequest = serde_json::from_str(&input).unwrap();

    // Parse input and return error if necessary
    let parsed_source: Result<Command, ParseError> = CustomHandler.parse(&request.program);
    if let Err(e) = parsed_source {
        let response = ExecutionResponse {
            result: "".to_string(),
            errors: vec![e.clone()],
        };

        return serde_json::to_string(&response).unwrap();
    }
    // TODO: Transform only if necessary
    let transformed_source = CustomHandler.transform(parsed_source.unwrap());

    // Transpile input
    let transpiled_source = &CustomHandler.transpile(transformed_source);
    // TODO: Call tool with transpiled
    //
    //

    // Respond with result
    serde_json::to_string(&transpiled_source).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![validate, execute])
}
