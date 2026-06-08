pub mod ast_visitor;
pub mod events;

pub use ast_visitor::AstVisitor;

use super::error::{Result, VisualizerError};
use syn::{parse_file, File};

pub fn parse_source(source: &str) -> Result<File> {
    parse_file(source).map_err(VisualizerError::ParseError)
}

pub fn read_and_parse_file(path: &str) -> Result<File> {
    let source = std::fs::read_to_string(path)?;
    parse_source(&source)
}
