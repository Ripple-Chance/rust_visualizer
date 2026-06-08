use thiserror::Error;

#[derive(Error, Debug)]
pub enum VisualizerError {
    #[error("Failed to read source file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse Rust source: {0}")]
    ParseError(#[from] syn::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, VisualizerError>;
