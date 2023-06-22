pub struct Respond;

use grammar::{ast::*, transpile::Transpile};

pub trait Responder {
    fn parse(&self, input: &str) -> Result<Command, String> {
        let parser = grammar::grammar::commandParser::new();
        let parsed = parser.parse(input);

        match parsed {
            Ok(ast) => Ok(*ast),
            Err(e) => Err(e.to_string()),
        }
    }
    fn transform(&self, ast: Command) -> Result<Command, String> {
        Ok(ast)
    }
    fn validate(&self, _ast: &Command) -> Result<(), String> {
        Ok(())
    }
    fn transpile(&self, ast: Command) -> String {
        struct Transpiler;
        impl grammar::transpile::Transpile for Transpiler {}

        Transpiler.transpile_command(ast)
    }
    fn handle(&self, input: &str) -> String {
        let parsed = self.parse(input);
        if let Err(e) = parsed {
            return e;
        }

        let ast = parsed.unwrap();
        let validated = self.validate(&ast);
        if let Err(e) = validated {
            return e;
        }

        let transformed = self.transform(ast);
        if let Err(e) = validated {
            return e;
        }

        self.transpile(transformed.unwrap())
    }
}

impl Responder for Respond {}
