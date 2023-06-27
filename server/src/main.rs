#[macro_use]
extern crate rocket;
extern crate lalrpop_util;

mod cors;
mod handler;

use grammar::ast::*;
use grammar::transform::Transform;
use grammar::transpile::Transpile;
use grammar::visit::Visit;
use handler::{CustomHandler, Handler, ParseError, RequestHandler};
use serde::{Deserialize, Serialize};

impl RequestHandler for CustomHandler {
    // TODO: Implement custom transpilation here
    // fn transpile(&self, ast: Command) -> String {
    //     struct Transpiler;
    //     impl grammar::transpile::Transpile for Transpiler {
    //     }
    //     Transpiler.transpile_command(ast)
    // }

    // TODO: Implement custom transformation here
    // fn transform(&self, ast: Command) -> Command {
    //     struct Transformer;
    //     impl grammar::transform::Transform for Transformer {
    //     }
    //     Transformer.transform_command(ast)
    // }

    // TODO: Implement custom visitation here
    // fn validate(&self, ast: &Command) -> Result<(), String> {
    //     struct Visitor;
    //     impl grammar::visit::Visit for Visitor {}
    //     Visitor.visit_command(ast);
    //     Ok()
    // }
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

//      Example chart data response:
//      Replace the ExecutionResponse.result field with this struct:

// #[derive(Serialize)]
// struct ChartData {
//     labels: Vec<String>,
//     datasets: Vec<ChartDataset>,
// }

// #[derive(Serialize)]
// struct ChartDataset {
//     label: String,
//     data: Vec<f64>,
// }

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

    // Transpile input using CustomHandler
    let transpiled_source = &CustomHandler.transpile(transformed_source);

    // TODO: Implement custom visitation or executions

    let response = ExecutionResponse {
        result: transpiled_source.to_string(),
        errors: vec![],
    };

    // Respond with result
    serde_json::to_string(&response).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(cors::CORS)
        .mount("/", routes![validate, execute])
}
