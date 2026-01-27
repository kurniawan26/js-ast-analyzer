use super::analyzers::Analyzers;
use crate::error::{AnalyzerError, Result};
use crate::types::{AnalysisResult, FileAnalysis, SeveritySummary};
use std::path::{Path, PathBuf};
use std::fs;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

pub struct JsParser {
    allocator: Allocator,
    analyzers: Analyzers,
}

impl JsParser {
    pub fn new() -> Self {
        Self {
            allocator: Allocator::default(),
            analyzers: Analyzers::new(),
        }
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<FileAnalysis> {
        let code = fs::read_to_string(file_path).map_err(|_| AnalyzerError::FileReadError {
            path: file_path.display().to_string(),
        })?;

        let source_type = SourceType::from_path(file_path).unwrap_or(SourceType::default());
        let parser = Parser::new(&self.allocator, &code, source_type);

        let ret = parser.parse();

        if !ret.errors.is_empty() {
             return Err(AnalyzerError::ParseError {
                file: file_path.display().to_string(),
                line: 0,
                column: 0,
                message: ret.errors[0].to_string(),
            });
        }

        let program = ret.program;

        let issues = self.analyzers.analyze_module(&program, file_path, &code);

        let mut summary = SeveritySummary::new();
        for issue in &issues {
            summary.add(issue.severity);
        }

        Ok(FileAnalysis {
            file_path: file_path.display().to_string(),
            issues,
            summary,
        })
    }

    pub fn analyze_directory(&self, dir_path: &Path) -> Result<AnalysisResult> {
        let mut result = AnalysisResult::new();

        let js_files = self.find_js_files(dir_path)?;

        for file_path in js_files {
            match self.analyze_file(&file_path) {
                Ok(file_analysis) => {
                    result.add_file(file_analysis);
                }
                Err(e) => {
                    eprintln!("Failed to analyze {}: {}", file_path.display(), e);
                }
            }
        }

        Ok(result)
    }

    fn find_js_files(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut js_files = Vec::new();

        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension().and_then(|e| e.to_str());
                if matches!(extension, Some("js") | Some("jsx") | Some("ts") | Some("tsx")) {
                    // Skip node_modules
                    if !path.to_string_lossy().contains("node_modules") {
                        js_files.push(path.to_path_buf());
                    }
                }
            }
        }

        Ok(js_files)
    }
}

impl Default for JsParser {
    fn default() -> Self {
        Self::new()
    }
}
