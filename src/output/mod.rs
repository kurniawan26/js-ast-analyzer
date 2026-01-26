use crate::types::{AnalysisResult, OutputFormat};
use colored::*;
use std::io::{self, Write};

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn format(result: &AnalysisResult, format: OutputFormat) -> String {
        match format {
            OutputFormat::Json => Self::format_json(result),
            OutputFormat::Human => Self::format_human(result),
        }
    }

    fn format_json(result: &AnalysisResult) -> String {
        serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
    }

    fn format_human(result: &AnalysisResult) -> String {
        let mut output = String::new();

        // Print summary
        output.push_str(&format!(
            "\n{}\n",
            "═══════════════════════════════════════".dimmed()
        ));
        output.push_str(&format!(
            "{}\n",
            "  Code Quality Analysis Results".bold().white()
        ));
        output.push_str(&format!(
            "{}\n\n",
            "═══════════════════════════════════════".dimmed()
        ));

        // Print overall summary
        output.push_str(&format!("{}: {}\n", "Total Files".bold(), result.files.len()));
        output.push_str(&format!(
            "{}: {} {}",
            "Total Issues".bold(),
            result.summary.total,
            Self::color_number(result.summary.total)
        ));
        output.push_str(&format!(
            " ({} errors, {} warnings, {} suggestions)\n\n",
            Self::color_count(result.summary.error, "error"),
            Self::color_count(result.summary.warning, "warning"),
            Self::color_count(result.summary.suggestion, "suggestion")
        ));

        // Print issues by file
        for file in &result.files {
            if file.issues.is_empty() {
                output.push_str(&format!(
                    "✓ {} - No issues found\n",
                    file.file_path.dimmed()
                ));
            } else {
                output.push_str(&format!("\n{}:\n", file.file_path.bold().cyan()));
                output.push_str(&format!(
                    "{}\n",
                    "─".repeat(80).dimmed()
                ));

                for issue in &file.issues {
                    let icon = match issue.severity {
                        crate::types::Severity::Error => "✖".red(),
                        crate::types::Severity::Warning => "⚠".yellow(),
                        crate::types::Severity::Suggestion => "ℹ".blue(),
                    };

                    output.push_str(&format!(
                        "  {} {}:{}:{}\n",
                        icon,
                        issue.line.to_string().dimmed(),
                        issue.column.to_string().dimmed(),
                        Self::severity_label(issue.severity)
                    ));
                    output.push_str(&format!(
                        "    {}\n",
                        issue.message.white()
                    ));
                    output.push_str(&format!(
                        "    [{}: {}]\n",
                        "rule".dimmed(),
                        issue.rule.dimmed()
                    ));

                    if let Some(snippet) = &issue.code_snippet {
                        output.push_str(&format!(
                            "    {}\n",
                            format!("> {}", snippet).dimmed()
                        ));
                    }
                    output.push('\n');
                }
            }
        }

        output
    }

    fn severity_label(severity: crate::types::Severity) -> ColoredString {
        match severity {
            crate::types::Severity::Error => "error".red().bold(),
            crate::types::Severity::Warning => "warning".yellow().bold(),
            crate::types::Severity::Suggestion => "suggestion".blue().bold(),
        }
    }

    fn color_number(n: usize) -> ColoredString {
        if n == 0 {
            n.to_string().green()
        } else if n < 5 {
            n.to_string().yellow()
        } else {
            n.to_string().red()
        }
    }

    fn color_count(count: usize, label: &str) -> String {
        let colored = if count == 0 {
            count.to_string().green()
        } else if count < 5 {
            count.to_string().yellow()
        } else {
            count.to_string().red()
        };
        format!("{} {}", colored, label)
    }

    pub fn print(result: &AnalysisResult, format: OutputFormat) {
        let output = Self::format(result, format);
        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}
