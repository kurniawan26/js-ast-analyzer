use thiserror::Error;

/// Error types for the AST analyzer
#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error in {file}:{line}:{column} - {message}")]
    ParseError { file: String, line: usize, column: usize, message: String },

    #[error("Failed to read file: {path}")]
    FileReadError { path: String },

    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

pub type Result<T> = std::result::Result<T, AnalyzerError>;
