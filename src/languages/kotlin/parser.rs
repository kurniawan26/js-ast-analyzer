use crate::types::{AnalysisResult, FileAnalysis, SeveritySummary, CodeIssue, Severity, Category};
use crate::error::{AnalyzerError, Result};
use std::path::{Path, PathBuf};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor};

pub struct KotlinParser {
}

impl KotlinParser {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<FileAnalysis> {
        let code = fs::read_to_string(file_path).map_err(|_| AnalyzerError::FileReadError {
            path: file_path.display().to_string(),
        })?;

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_kotlin::language()).expect("Error loading Kotlin grammar");

        let tree = parser.parse(&code, None).ok_or_else(|| AnalyzerError::ParseError {
            file: file_path.display().to_string(),
            line: 0,
            column: 0,
            message: "Failed to parse Kotlin file".to_string(),
        })?;

        let mut issues = Vec::new();

        // 1. Check for syntax errors (ERROR nodes)
        let root_node = tree.root_node();
        if root_node.has_error() {
             issues.push(CodeIssue {
                file_path: file_path.display().to_string(),
                line: 1, // Simplified
                column: 1,
                end_line: None,
                end_column: None,
                message: "Syntax error detected in Kotlin file".to_string(),
                severity: Severity::Error,
                category: Category::CodeQuality,
                rule: "kotlin-syntax-error".to_string(),
                code_snippet: None,
            });
        }

        // 2. Custom Rule: Avoid println
        let query_source = "
            (call_expression (simple_identifier) @function_name (#match? @function_name \"^print(ln)?$\")) @print_call
            
            (integer_literal) @magic_number
            
            (class_declaration (type_identifier) @class_name)
            
            (property_declaration (variable_declaration (simple_identifier) @variable_name))
            
            (if_expression) @if_stmt

            (property_declaration (variable_declaration (simple_identifier) @unused_variable)) 

        ";
        
        let query = Query::new(&tree_sitter_kotlin::language(), query_source).unwrap();
        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&query, root_node, code.as_bytes());

        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let start_position = node.start_position();
                let end_position = node.end_position();
                let capture_name = query.capture_names()[capture.index as usize];
                
                let issue = match capture_name {
                    "print_call" => Some(CodeIssue {
                        file_path: file_path.display().to_string(),
                        line: start_position.row + 1,
                        column: start_position.column + 1,
                        end_line: Some(end_position.row + 1),
                        end_column: Some(end_position.column + 1),
                        message: "Avoid using print/println in production code. Use a logger instead.".to_string(),
                        severity: Severity::Warning,
                        category: Category::BestPractice,
                        rule: "no-print".to_string(),
                        code_snippet: Some(node.utf8_text(code.as_bytes()).unwrap().to_string()),
                    }),
                    "magic_number" => {
                        let text = node.utf8_text(code.as_bytes()).unwrap();
                         if text != "0" && text != "1" && text != "-1" {
                            Some(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start_position.row + 1,
                                column: start_position.column + 1,
                                end_line: Some(end_position.row + 1),
                                end_column: Some(end_position.column + 1),
                                message: format!("Magic number detected: {}. Define as a constant.", text),
                                severity: Severity::Suggestion,
                                category: Category::BestPractice,
                                rule: "no-magic-numbers".to_string(),
                                code_snippet: Some(text.to_string()),
                            })
                        } else {
                            None
                        }
                    },
                     "class_name" => {
                        let text = node.utf8_text(code.as_bytes()).unwrap();
                         if !text.chars().next().map_or(false, |c| c.is_uppercase()) {
                            Some(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start_position.row + 1,
                                column: start_position.column + 1,
                                end_line: Some(end_position.row + 1),
                                end_column: Some(end_position.column + 1),
                                message: format!("Class name '{}' should start with an uppercase letter.", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "class-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            })
                        } else {
                            None
                        }
                    },
                    "variable_name" => {
                         let text = node.utf8_text(code.as_bytes()).unwrap();
                         if !text.chars().next().map_or(false, |c| c.is_lowercase()) {
                            Some(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start_position.row + 1,
                                column: start_position.column + 1,
                                end_line: Some(end_position.row + 1),
                                end_column: Some(end_position.column + 1),
                                message: format!("Variable name '{}' should start with a lowercase letter.", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "variable-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            })
                        } else {
                            None
                        }
                    },
                    "unused_variable" => {
                        let text = node.utf8_text(code.as_bytes()).unwrap();
                        
                        // Very basic check: search if the variable name appears elsewhere in the file
                        // This is NOT accurate scope analysis but a heuristic for this demo
                        let count = code.matches(text).count();
                        if count <= 1 { // Only declaration
                             Some(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start_position.row + 1,
                                column: start_position.column + 1,
                                end_line: Some(end_position.row + 1),
                                end_column: Some(end_position.column + 1),
                                message: format!("Variable '{}' appears to be unused.", text),
                                severity: Severity::Warning,
                                category: Category::Maintainability,
                                rule: "unused-variable".to_string(),
                                code_snippet: Some(text.to_string()),
                            })
                        } else {
                            None
                        }
                    },
                     "null_value" =>  Some(CodeIssue {
                        file_path: file_path.display().to_string(),
                        line: start_position.row + 1,
                        column: start_position.column + 1,
                        end_line: Some(end_position.row + 1),
                        end_column: Some(end_position.column + 1),
                        message: "Avoid using 'null' directly. Use Kotlin's null safety features like '?' or '?:'.".to_string(),
                        severity: Severity::Suggestion,
                        category: Category::CodeQuality,
                        rule: "avoid-null".to_string(),
                        code_snippet: Some("null".to_string()),
                    }),
                     "nested_if" => { /* Removed specific query logic, handled by generic if_stmt */ None }, // Keep for compatibility if I revert
                     "if_stmt" => {
                        // Check nesting level manually
                        let mut depth = 0;
                        let mut parent = node.parent();
                        while let Some(p) = parent {
                            if p.kind() == "if_expression" {
                                depth += 1;
                            }
                            parent = p.parent();
                        }
                        
                        if depth >= 2 {
                             Some(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start_position.row + 1,
                                column: start_position.column + 1,
                                end_line: Some(end_position.row + 1),
                                end_column: Some(end_position.column + 1),
                                message: "Avoid deeply nested if statements (depth >= 2). Refactor into smaller functions.".to_string(),
                                severity: Severity::Warning,
                                category: Category::Complexity,
                                rule: "nested-if".to_string(),
                                code_snippet: Some("if (...)".to_string()),
                            })
                        } else {
                            None
                        }
                     },
                    _ => None,
                };

                if let Some(issue) = issue {
                    issues.push(issue);
                }
            }
        }

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

        let kt_files = self.find_kt_files(dir_path)?;

        for file_path in kt_files {
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

    fn find_kt_files(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut kt_files = Vec::new();

        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                let extension = path.extension().and_then(|e| e.to_str());
                if matches!(extension, Some("kt") | Some("kts")) {
                    kt_files.push(path.to_path_buf());
                }
            }
        }

        Ok(kt_files)
    }
}

impl Default for KotlinParser {
    fn default() -> Self {
        Self::new()
    }
}
