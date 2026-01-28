use crate::types::{AnalysisResult, FileAnalysis, SeveritySummary, CodeIssue, Severity, Category};
use crate::error::{AnalyzerError, Result};
use std::path::{Path, PathBuf};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor};

pub struct PythonParser {}

impl PythonParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<FileAnalysis> {
        let code = fs::read_to_string(file_path).map_err(|_| AnalyzerError::FileReadError {
            path: file_path.display().to_string(),
        })?;

        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_python::language()).expect("Error loading Python grammar");

        let tree = parser.parse(&code, None).ok_or_else(|| AnalyzerError::ParseError {
            file: file_path.display().to_string(),
            line: 0,
            column: 0,
            message: "Failed to parse Python file".to_string(),
        })?;

        let mut issues = Vec::new();
        let root_node = tree.root_node();
        println!("Python AST: {}", root_node.to_sexp());

        // Check for syntax errors
        if root_node.has_error() {
            issues.push(CodeIssue {
                file_path: file_path.display().to_string(),
                line: 1,
                column: 1,
                end_line: None,
                end_column: None,
                message: "Syntax error detected in Python file".to_string(),
                severity: Severity::Error,
                category: Category::CodeQuality,
                rule: "python-syntax-error".to_string(),
                code_snippet: None,
            });
        }

        // Queries for Python
        let query_source = "
            (call
                function: (identifier) @func_name
                arguments: (argument_list)
                (#match? @func_name \"^print$\")
            ) @print_call

            (integer) @magic_number
            (float) @magic_number
            
            (string) @string_literal

            (class_definition
                name: (identifier) @class_name
            )

            (function_definition
                name: (identifier) @def_func_name
            )

            (assignment
                left: (identifier) @var_assign
            )
            
            (if_statement) @if_stmt
            
            (function_definition
                parameters: (parameters) @params
            ) @func_def_params
        ";

        if let Ok(query) = Query::new(&tree_sitter_python::language(), query_source) {
            let mut query_cursor = QueryCursor::new();
            let matches = query_cursor.matches(&query, root_node, code.as_bytes());

            for m in matches {
                for capture in m.captures {
                    let node = capture.node;
                    let start = node.start_position();
                    let end = node.end_position();
                    let capture_name = query.capture_names()[capture.index as usize];

                    if capture_name == "print_call" {
                        issues.push(CodeIssue {
                            file_path: file_path.display().to_string(),
                            line: start.row + 1,
                            column: start.column + 1,
                            end_line: Some(end.row + 1),
                            end_column: Some(end.column + 1),
                            message: "Avoid using print() in production. Use a logger.".to_string(),
                            severity: Severity::Warning,
                            category: Category::BestPractice,
                            rule: "no-print".to_string(),
                            code_snippet: Some(node.utf8_text(code.as_bytes()).unwrap_or("").to_string()),
                        });
                    } else if capture_name == "magic_number" {
                         let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                         // Ignore common small numbers
                         if text != "0" && text != "1" && text != "-1" && text != "2" && text != "10" && text != "100" {
                             // Ignore if it's in a CONSTANT assignment (UPPERCASE variable) or default parameter value
                             let mut is_const = false;
                             let mut parent = node.parent();
                             while let Some(p) = parent {
                                 if p.kind() == "assignment" {
                                     // Check if left side is UPPERCASE
                                     if let Some(left) = p.child_by_field_name("left") {
                                         if let Ok(name) = left.utf8_text(code.as_bytes()) {
                                             if name.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) {
                                                 is_const = true;
                                             }
                                         }
                                     }
                                 } else if p.kind() == "keyword_argument" || p.kind() == "default_parameter" {
                                     // Allow magic numbers as default values? Maybe not, but often acceptable.
                                     // Let's stick strictly to 'define a constant'.
                                 }
                                 
                                 if is_const { break; }
                                 parent = p.parent();
                                 if parent.is_none() || p.kind() == "function_definition" || p.kind() == "class_definition" {
                                     break;
                                 }
                             }

                             if !is_const {
                                issues.push(CodeIssue {
                                    file_path: file_path.display().to_string(),
                                    line: start.row + 1,
                                    column: start.column + 1,
                                    end_line: Some(end.row + 1),
                                    end_column: Some(end.column + 1),
                                    message: format!("magic number detected: {}. Define a constant.", text),
                                    severity: Severity::Suggestion,
                                    category: Category::BestPractice,
                                    rule: "no-magic-numbers".to_string(),
                                    code_snippet: Some(text.to_string()),
                                });
                             }
                         }
                    } else if capture_name == "string_literal" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        let clean_text = text.trim_matches(&['\'', '"'][..]);
                        if clean_text.len() > 20 && !clean_text.contains("{") {
                             // Ignore docstrings
                             let mut is_docstring = false;
                             if let Some(parent) = node.parent() {
                                 if parent.kind() == "expression_statement" {
                                     if let Some(grandparent) = parent.parent() {
                                         if grandparent.kind() == "function_definition" || grandparent.kind() == "class_definition" || grandparent.kind() == "module" {
                                              // It's likely a docstring if it's the first statement
                                              let mut cursor = grandparent.walk();
                                              for child in grandparent.children(&mut cursor) {
                                                  if child.kind() == "expression_statement" {
                                                      if child == parent {
                                                          is_docstring = true;
                                                      }
                                                      break; // Only check the first one
                                                  } else if child.kind() == "comment" {
                                                      continue;
                                                  } else {
                                                       // If we hit something else before our node, it's not a docstring (usually)
                                                      if child.start_position().row < parent.start_position().row {
                                                          // preceeding code
                                                      }
                                                  }
                                              }
                                         }
                                     }
                                 }
                             }
                             
                             if !is_docstring {
                                issues.push(CodeIssue {
                                    file_path: file_path.display().to_string(),
                                    line: start.row + 1,
                                    column: start.column + 1,
                                    end_line: Some(end.row + 1),
                                    end_column: Some(end.column + 1),
                                    message: format!("hardcoded string detected: \"{}...\". Consider extracting to a constant.", &clean_text.chars().take(20).collect::<String>()),
                                    severity: Severity::Suggestion,
                                    category: Category::BestPractice,
                                    rule: "no-hardcoded-strings".to_string(),
                                    code_snippet: Some(clean_text.to_string()),
                                });
                             }
                        }
                    } else if capture_name == "class_name" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        if !text.chars().next().map_or(false, |c| c.is_uppercase()) {
                             issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Class name '{}' should be PascalCase.", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "class-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            });
                        }
                    } else if capture_name == "def_func_name" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        if !text.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric()) {
                             issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Function name '{}' should be snake_case.", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "function-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            });
                        }
                    } else if capture_name == "var_assign" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        
                        // Check for snake_case
                        // Allow CONSTANTS (all uppercase)
                        let is_constant = text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
                        if !is_constant && !text.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric()) {
                             issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Variable name '{}' should be snake_case (or UPPER_CASE for constants).", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "variable-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            });
                        }
                    } else if capture_name == "if_stmt" {
                        // Check nesting
                        let mut depth = 0;
                        let mut parent = node.parent();
                        while let Some(p) = parent {
                            if p.kind() == "if_statement" {
                                depth += 1;
                            }
                            parent = p.parent();
                        }
                        if depth >= 2 {
                             issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: "Avoid deeply nested if statements.".to_string(),
                                severity: Severity::Warning,
                                category: Category::Complexity,
                                rule: "nested-if".to_string(),
                                code_snippet: Some("if ...".to_string()),
                            });
                        }
                    } else if capture_name == "params" {
                        // Check parameter count
                        let param_count = node.child_count();
                        // Tree-sitter 'parameters' node contains '(' and ')' and commas.
                        // We count actual identifiers or named parameters.
                        let mut actual_params = 0;
                        let mut cursor = node.walk();
                        for child in node.children(&mut cursor) {
                            if child.kind() == "identifier" || child.kind() == "typed_parameter" || child.kind() == "default_parameter" || child.kind() == "typed_default_parameter" {
                                actual_params += 1;
                            }
                        }

                        if actual_params > 5 {
                             issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Function has too many parameters ({}). Max allowed is 5.", actual_params),
                                severity: Severity::Warning,
                                category: Category::Complexity,
                                rule: "complexity".to_string(),
                                code_snippet: Some("def func(...)".to_string()),
                            });
                        }
                    }
                }
            }
        } else if let Err(e) = Query::new(&tree_sitter_python::language(), query_source) {
             issues.push(CodeIssue {
                file_path: file_path.display().to_string(),
                line: 1,
                column: 1,
                end_line: None,
                end_column: None,
                message: format!("Internal Error: Failed to compile Python AST Query: {}", e),
                severity: Severity::Error,
                category: Category::CodeQuality,
                rule: "internal-error".to_string(),
                code_snippet: None,
            });
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
        let files = self.find_files(dir_path)?;

        for file_path in files {
             if let Ok(analysis) = self.analyze_file(&file_path) {
                 result.add_file(analysis);
             }
        }
        Ok(result)
    }

    fn find_files(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "py" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
        Ok(files)
    }
}

impl Default for PythonParser {
    fn default() -> Self {
        Self::new()
    }
}
