// Library exports for testing and external use

pub mod analyzers;
pub mod error;
pub mod output;
pub mod parser;
pub mod types;

// Re-export commonly used types
pub use error::{AnalyzerError, Result};
pub use parser::JsParser;
pub use types::{AnalysisResult, FileAnalysis, CodeIssue as Issue, Severity, OutputFormat};
