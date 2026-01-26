use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod analyzers;
mod error;
mod output;
mod parser;
mod types;

use error::AnalyzerError;
use output::OutputFormatter;
use parser::JsParser;
use types::OutputFormat;

/// JavaScript/TypeScript AST Analyzer for Code Quality
#[derive(Parser, Debug)]
#[command(name = "js-ast-analyzer")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "Analyze JavaScript/TypeScript code for quality and security issues", long_about = None)]
struct Args {
    /// Path to file or directory to analyze
    #[arg(value_name = "PATH")]
    path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Exit with error code if any issues are found
    #[arg(short, long)]
    strict: bool,

    /// Filter issues by severity (error, warning, suggestion)
    #[arg(short = 'S', long)]
    severity: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate path exists
    if !args.path.exists() {
        return Err(AnalyzerError::InvalidPath(args.path.display().to_string()).into());
    }

    // Initialize parser
    let parser = JsParser::new();

    // Analyze
    let result = if args.path.is_file() {
        let file_analysis = parser.analyze_file(&args.path)?;
        let mut analysis_result = types::AnalysisResult::new();
        analysis_result.add_file(file_analysis);
        analysis_result
    } else {
        parser.analyze_directory(&args.path)?
    };

    // Print results
    OutputFormatter::print(&result, args.format);

    // Exit code for strict mode
    if args.strict && result.summary.total > 0 {
        std::process::exit(1);
    }

    Ok(())
}
