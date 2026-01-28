use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity level of a code issue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Suggestion,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Suggestion => write!(f, "suggestion"),
        }
    }
}

/// Category of the code issue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Security,
    BestPractice,
    CodeQuality,
    Performance,
    Maintainability,
    TypeScript,
    Kotlin,
    Complexity,
    Dart,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Security => write!(f, "security"),
            Category::BestPractice => write!(f, "best-practice"),
            Category::CodeQuality => write!(f, "code-quality"),
            Category::Performance => write!(f, "performance"),
            Category::Maintainability => write!(f, "maintainability"),
            Category::TypeScript => write!(f, "typescript"),
            Category::Kotlin => write!(f, "kotlin"),
            Category::Complexity => write!(f, "complexity"),
            Category::Dart => write!(f, "dart"),
        }
    }
}

/// A code issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    /// File path where the issue was found
    pub file_path: String,

    /// Line number (1-indexed)
    pub line: usize,

    /// Column number (1-indexed)
    pub column: usize,

    /// End line number (1-indexed, if multiline)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<usize>,

    /// End column number (1-indexed, if multiline)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<usize>,

    /// Descriptive message about the issue
    pub message: String,

    /// Severity level
    pub severity: Severity,

    /// Category of the issue
    pub category: Category,

    /// Rule identifier that triggered this issue
    pub rule: String,

    /// Code snippet that triggered the issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<String>,
}

/// Summary of issues by severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeveritySummary {
    pub error: usize,
    pub warning: usize,
    pub suggestion: usize,
    pub total: usize,
}

impl SeveritySummary {
    pub fn new() -> Self {
        Self {
            error: 0,
            warning: 0,
            suggestion: 0,
            total: 0,
        }
    }

    pub fn add(&mut self, severity: Severity) {
        match severity {
            Severity::Error => self.error += 1,
            Severity::Warning => self.warning += 1,
            Severity::Suggestion => self.suggestion += 1,
        }
        self.total += 1;
    }
}

impl Default for SeveritySummary {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis result for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub file_path: String,
    pub issues: Vec<CodeIssue>,
    pub summary: SeveritySummary,
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub files: Vec<FileAnalysis>,
    pub summary: SeveritySummary,
}

impl AnalysisResult {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            summary: SeveritySummary::new(),
        }
    }

    pub fn add_file(&mut self, file_analysis: FileAnalysis) {
        self.summary.error += file_analysis.summary.error;
        self.summary.warning += file_analysis.summary.warning;
        self.summary.suggestion += file_analysis.summary.suggestion;
        self.summary.total += file_analysis.summary.total;
        self.files.push(file_analysis);
    }
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Human,
}

/// Programming language options
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Language {
    Javascript,
    Typescript,
    Python,
    Kotlin,
    Dart,
}



