#[macro_use]
extern crate rocket;

#[post("/parse_grammar", data = "<input>")]
fn parse_grammar(input: String) -> String {
    match grammar::parse_grammar(&input) {
        Ok(ast) => ast.to_string(),
        Err(e) => e,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![parse_grammar])
}
