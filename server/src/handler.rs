#![allow(non_snake_case)]

use grammar::ast::*;
use grammar::grammar::commandParser;
use grammar::transpile::Transpile;
use lalrpop_util::ParseError as LalrpopParseError;

use lalrpop_util::lexer::Token;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ParseError {
    startLineNumber: usize,
    startColumn: usize,
    endLineNumber: usize,
    endColumn: usize,
    message: String,
    severity: usize,
}

impl ParseError {
    fn new(
        startLineNumber: usize,
        startColumn: usize,
        endLineNumber: usize,
        endColumn: usize,
        message: String,
    ) -> Self {
        Self {
            startLineNumber,
            startColumn,
            endLineNumber,
            endColumn,
            message,
            severity: 8,
        }
    }

    fn full_sized_error(input: &str, error: String) -> Self {
        let (line, column) = index_to_line_column(&input, input.len()).unwrap();
        Self::new(1, 1, line, column, error)
    }

    fn single_char_error(input: &str, error: String, index: usize) -> Self {
        let (line, column) = index_to_line_column(&input, index).unwrap();
        Self::new(line, column, line + 1, column, error)
    }

    fn location_based_error(input: &str, error: String, start: usize, end: usize) -> Self {
        let (lineStart, columnStart) = index_to_line_column(input, start).unwrap();
        let (lineEnd, columnEnd) = index_to_line_column(input, end).unwrap();
        Self::new(lineStart, columnStart, lineEnd, columnEnd, error)
    }
}

fn index_to_line_column(string: &str, index: usize) -> Option<(usize, usize)> {
    let mut line = 1;
    let mut column = 1;
    let mut current_index = 0;

    for c in string.chars() {
        if current_index == index {
            return Some((line, column));
        }

        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }

        current_index += 1;
    }

    if current_index == index {
        Some((line, column))
    } else {
        None
    }
}

pub trait RequestHandler {
    fn parse(&self, input: &str) -> Result<Command, ParseError> {
        let parser = commandParser::new();
        let parsed = parser.parse(input);

        match parsed {
            Ok(ast) => Ok(*ast),
            Err(e) => Err(convert_to_parse_error(input, e)),
        }
    }
    fn transform(&self, ast: Command) -> Result<Command, String> {
        Ok(ast)
    }
    fn visit(&self, _ast: &Command) -> Result<(), String> {
        Ok(())
    }
    fn transpile(&self, ast: Command) -> String {
        struct Transpiler;
        impl grammar::transpile::Transpile for Transpiler {}

        Transpiler.transpile_command(ast)
    }
    fn handle(&self, _input: &str) -> Result<(), ParseError> {
        Ok(())
    }

    fn validate(&self, input: &str) -> Result<(), ParseError> {
        let parsed = self.parse(input);
        if let Err(e) = parsed {
            return Err(e);
        }

        let ast = parsed.unwrap();
        let validated = self.visit(&ast);
        if let Err(e) = validated {
            return Err(ParseError::full_sized_error(input, e));
        }

        Ok(())
    }
}

fn convert_to_parse_error(
    input: &str,
    e: LalrpopParseError<usize, Token, &'static str>,
) -> ParseError {
    let error_message = e.to_string();
    match e {
        LalrpopParseError::InvalidToken { location } => {
            ParseError::single_char_error(input, error_message, location)
        }
        LalrpopParseError::UnrecognizedEof {
            location,
            expected: _,
        } => ParseError::single_char_error(input, error_message, location),

        LalrpopParseError::UnrecognizedToken { token, expected: _ } => {
            ParseError::location_based_error(input, error_message, token.0, token.2)
        }
        LalrpopParseError::ExtraToken { token } => {
            ParseError::location_based_error(input, error_message, token.0, token.2)
        }
        _ => unreachable!(),
    }
}
