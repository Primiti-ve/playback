#![allow(unused, clippy::manual_strip)]

pub mod ast;
pub mod engine;
pub mod error;
pub mod parser;

pub use ast::{Expr, Value};
pub use error::ParseError;

use std::collections::HashMap;

pub fn parse(input: &str) -> Result<HashMap<String, Value>, ParseError> {
    parser::Parser::new(input).parse()
}
