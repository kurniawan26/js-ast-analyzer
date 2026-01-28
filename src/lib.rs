// Library exports for testing and external use

pub mod languages;
pub mod error;
pub mod output;
pub mod types;

// Re-export commonly used types
pub use error::{AnalyzerError, Result};
pub use languages::javascript::JsParser;
pub use languages::kotlin::KotlinParser;
pub use languages::dart::DartParser;
pub use types::{AnalysisResult, FileAnalysis, CodeIssue as Issue, Severity, OutputFormat};
