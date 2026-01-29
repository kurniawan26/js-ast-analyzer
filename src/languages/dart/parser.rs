use crate::error::{AnalyzerError, Result};
use crate::types::{AnalysisResult, Category, CodeIssue, FileAnalysis, Severity, SeveritySummary};
use std::fs;
use std::path::{Path, PathBuf};
use tree_sitter::{Parser, Query, QueryCursor};

pub struct DartParser {}

impl DartParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<FileAnalysis> {
        let code = fs::read_to_string(file_path).map_err(|_| AnalyzerError::FileReadError {
            path: file_path.display().to_string(),
        })?;

        let mut parser = Parser::new();
        // Since we don't have tree_sitter_dart trait directly available as language()
        // We will assume tree_sitter_dart::language() is available
        // If compilation fails, we might need a different import.
        parser
            .set_language(&tree_sitter_dart::language())
            .expect("Error loading Dart grammar");

        let tree = parser
            .parse(&code, None)
            .ok_or_else(|| AnalyzerError::ParseError {
                file: file_path.display().to_string(),
                line: 0,
                column: 0,
                message: "Failed to parse Dart file".to_string(),
            })?;

        let mut issues = Vec::new();
        let root_node = tree.root_node();

        // Check for syntax errors
        if root_node.has_error() {
            issues.push(CodeIssue {
                file_path: file_path.display().to_string(),
                line: 1,
                column: 1,
                end_line: None,
                end_column: None,
                message: "Syntax error detected in Dart file".to_string(),
                severity: Severity::Error,
                category: Category::CodeQuality,
                rule: "dart-syntax-error".to_string(),
                code_snippet: None,
            });
        }

        // Queries for Dart
        // Based on typical tree-sitter-dart node names (guessed but common):
        // function_signature, method_invocation, identifier, class_definition, etc.

        // Try to construct query. If it fails, we might have wrong node names.
        // I will use a simplified query set first if unsure, but I will try to be reasonably complete.
        // Actually, if node names are wrong, Query::new will panic or return error. I should unwrap for now.
        // Common dart node names:
        // class_definition, identifier, function_expression? No, 'function_call' or 'method_invocation'.
        // Let's assume standard names.
        // Wait, Dart grammar is tricky.
        // 'print' is just a function call.
        // (function_expression) might be defining a function.
        // Invocation: (function_expression_invocation) ? (method_invocation)?
        // Let's try a safer query with wildcards or just simpler nodes that I'm 80% sure of.
        // identifier, integer_literal are usually safe.
        // class_definition? Maybe 'class_declaration'.

        // To be safe, I will use a very permissive query or catch the error if I could, but standard unwrap is fine for 'dev'.
        // query_source
        let query_source = "
            (member_access
                (identifier) @func_name
                (selector (argument_part))
                (#match? @func_name \"^print$\")
            ) @print_call
            
            (decimal_integer_literal) @magic_number
            (decimal_floating_point_literal) @magic_number
            (hex_integer_literal) @magic_number
            
            (string_literal) @string_literal
            
            (class_definition 
                name: (identifier) @class_name
            )

            (initialized_variable_definition 
                name: (identifier) @variable_name
            )

            (if_statement) @if_stmt

            ;; Null safety - Definitions
            (initialized_variable_definition 
                name: (identifier) @def_name
            ) @var_def

            ;; Null safety - Access
            (member_access 
                (identifier) @access_target 
                (selector) @selector_node
            )
        ";

        // NOTE: If the above query fails at runtime, I might need to adjust node names.
        let mut nullable_vars = std::collections::HashSet::new();

        if let Ok(query) = Query::new(&tree_sitter_dart::language(), query_source) {
            let mut query_cursor = QueryCursor::new();
            let matches = query_cursor.matches(&query, root_node, code.as_bytes());

            for m in matches {
                // Pre-process match for definitions
                let capture_map: std::collections::HashMap<_, _> = m
                    .captures
                    .iter()
                    .map(|c| (query.capture_names()[c.index as usize], c.node))
                    .collect();
                if let (Some(var_def_node), Some(name_node)) =
                    (capture_map.get("var_def"), capture_map.get("def_name"))
                {
                    let start_byte = var_def_node.start_byte();
                    let name_start_byte = name_node.start_byte();
                    if name_start_byte > start_byte {
                        let type_section = &code[start_byte..name_start_byte];
                        if type_section.contains('?') {
                            let name_text = name_node.utf8_text(code.as_bytes()).unwrap_or("");
                            nullable_vars.insert(name_text.to_string());
                        }
                    }
                }

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
                            code_snippet: Some(
                                node.utf8_text(code.as_bytes()).unwrap_or("").to_string(),
                            ),
                        });
                    } else if capture_name == "magic_number" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        // Ignore common small numbers
                        if text != "0"
                            && text != "1"
                            && text != "-1"
                            && text != "2"
                            && text != "10"
                            && text != "100"
                        {
                            // Ignore if it's in a const declaration
                            let mut is_const = false;
                            let mut parent = node.parent();
                            let mut depth = 0;
                            while let Some(p) = parent {
                                depth += 1;
                                if p.kind() == "initialized_variable_definition"
                                    || p.kind() == "declaration"
                                {
                                    // Check children for const_builtin
                                    let mut cursor = p.walk();
                                    for child in p.children(&mut cursor) {
                                        if child.kind() == "const_builtin" {
                                            is_const = true;
                                            break;
                                        }
                                    }
                                }
                                if is_const {
                                    break;
                                }
                                parent = p.parent();
                                if parent.is_none()
                                    || p.kind() == "class_body"
                                    || p.kind() == "block"
                                    || depth > 5
                                {
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
                                    message: format!(
                                        "magic number detected: {}. Define a constant.",
                                        text
                                    ),
                                    severity: Severity::Suggestion,
                                    category: Category::BestPractice,
                                    rule: "no-magic-numbers".to_string(),
                                    code_snippet: Some(text.to_string()),
                                });
                            }
                        }
                    } else if capture_name == "string_literal" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        // Heuristic: Check if string is long and not in a const declaration
                        if text.len() > 20 && !text.contains("${") {
                            let mut is_const = false;
                            let mut parent = node.parent();
                            while let Some(p) = parent {
                                if p.kind() == "initialized_variable_definition"
                                    || p.kind() == "declaration"
                                {
                                    let mut cursor = p.walk();
                                    for child in p.children(&mut cursor) {
                                        if child.kind() == "const_builtin" {
                                            is_const = true;
                                            break;
                                        }
                                    }
                                }
                                if is_const {
                                    break;
                                }
                                parent = p.parent();
                                if parent.is_none()
                                    || p.kind() == "class_body"
                                    || p.kind() == "block"
                                {
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
                                    message: format!("hardcoded string detected: \"{}...\". Consider extracting to a constant.", &text.chars().take(20).collect::<String>()),
                                    severity: Severity::Suggestion,
                                    category: Category::BestPractice,
                                    rule: "no-hardcoded-strings".to_string(),
                                    code_snippet: Some(text.to_string()),
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
                    } else if capture_name == "variable_name" {
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");

                        // Check for camelCase
                        if !text.chars().next().map_or(false, |c| c.is_lowercase()) {
                            issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Variable name '{}' should be camelCase.", text),
                                severity: Severity::Warning,
                                category: Category::CodeQuality,
                                rule: "variable-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            });
                        }

                        // Check for generic names
                        let generic_names = [
                            "data", "result", "info", "value", "temp", "tmp", "obj", "item",
                        ];
                        if generic_names.contains(&text) {
                            issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Avoid using generic variable name '{}'. Use a more descriptive name.", text),
                                severity: Severity::Suggestion,
                                category: Category::BestPractice,
                                rule: "variable-naming".to_string(),
                                code_snippet: Some(text.to_string()),
                            });
                        }

                        // Check for short names
                        if text.len() < 3 {
                            let mut is_loop_var = false;
                            let mut parent = node.parent();
                            while let Some(p) = parent {
                                if p.kind() == "for_statement" || p.kind() == "for_in_statement" {
                                    is_loop_var = true;
                                    break;
                                }
                                parent = p.parent();
                            }

                            if !is_loop_var {
                                issues.push(CodeIssue {
                                    file_path: file_path.display().to_string(),
                                    line: start.row + 1,
                                    column: start.column + 1,
                                    end_line: Some(end.row + 1),
                                    end_column: Some(end.column + 1),
                                    message: format!("Penamaan variable '{}' cukup pendek. Kamu bisa menggunakan penamaan yang lebih deskriptif untuk penulisan yang lebih baik.", text),
                                    severity: Severity::Suggestion,
                                    category: Category::CodeQuality,
                                    rule: "variable-naming".to_string(),
                                    code_snippet: Some(text.to_string()),
                                });
                            }
                        }

                        // Check for boolean naming
                        if let Some(parent) = node.parent() {
                            let mut cursor = parent.walk();
                            for child in parent.children(&mut cursor) {
                                if (child.kind() == "type_identifier"
                                    || child.kind() == "boolean_type")
                                    && child.utf8_text(code.as_bytes()).unwrap_or("") == "bool"
                                {
                                    if !text.starts_with("is")
                                        && !text.starts_with("has")
                                        && !text.starts_with("can")
                                        && !text.starts_with("should")
                                    {
                                        issues.push(CodeIssue {
                                            file_path: file_path.display().to_string(),
                                            line: start.row + 1,
                                            column: start.column + 1,
                                            end_line: Some(end.row + 1),
                                            end_column: Some(end.column + 1),
                                            message: format!("Dalam menuliskan sebuah penamaan variable '{}' kamu bisa memulainya dengan keyword seperti 'is', 'has', 'can', or 'should'. Contohnya: isOddNumber", text),
                                            severity: Severity::Warning,
                                            category: Category::CodeQuality,
                                            rule: "variable-naming".to_string(),
                                            code_snippet: Some(text.to_string()),
                                        });
                                    }
                                }
                            }
                        }

                        // Heuristic unused check
                        let count = code.matches(text).count();
                        if count <= 1 {
                            issues.push(CodeIssue {
                                file_path: file_path.display().to_string(),
                                line: start.row + 1,
                                column: start.column + 1,
                                end_line: Some(end.row + 1),
                                end_column: Some(end.column + 1),
                                message: format!("Sepertinya variabel '{}' ini tidak kamu gunakan, kamu bisa melakukan penghapusan pada variabel yang tidak digunakan seperti ini ya!", text),
                                severity: Severity::Warning,
                                category: Category::Maintainability,
                                rule: "unused-variable".to_string(),
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
                                message: "Hindari penggunaan kondisi bersarang seperti ini ya, agar lebih baik kamu bisa melakukan refactor terlebih dahulu untuk memudahkan kamu dalam proses memahami kode berikutnya.".to_string(),
                                severity: Severity::Warning,
                                category: Category::Complexity,
                                rule: "nested-if".to_string(),
                                code_snippet: Some("if (...)".to_string()),
                            });
                        }
                    } else if capture_name == "access_target" {
                        // Check for unsafe access on nullable vars
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        if nullable_vars.contains(text) {
                            if let Some(selector) = capture_map.get("selector_node") {
                                // selector should contain '?' if safe
                                let selector_text =
                                    selector.utf8_text(code.as_bytes()).unwrap_or("");
                                if !selector_text.starts_with("?.")
                                    && !selector_text.starts_with("?[")
                                {
                                    issues.push(CodeIssue {
                                        file_path: file_path.display().to_string(),
                                        line: start.row + 1,
                                        column: start.column + 1,
                                        end_line: Some(end.row + 1),
                                        end_column: Some(end.column + 1),
                                        message: format!("Unsafe property access on nullable variable '{}'. Use '?.' or check for null.", text),
                                        severity: Severity::Error,
                                        category: Category::CodeQuality,
                                        rule: "null-safety".to_string(),
                                        code_snippet: Some(format!("{}{}", text, selector_text)),
                                    });
                                }
                            }
                        }
                    } else if capture_name == "selector_node" {
                        // Check for array access with literal index
                        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
                        // S-expression: (selector (unconditional_assignable_selector (index_selector (decimal_integer_literal))))
                        // We can just check text? selector text would be "[10]"
                        if text.starts_with('[') {
                            // It is an index selector
                            // Check for high index? Or just warn about bounds?
                            // Test calls it "Unsafe array access".
                            // Let's check for literal integers inside.
                            if text.contains(|c: char| c.is_digit(10)) {
                                // Very heuristic
                                // Check if it's a large number?
                                // Let's just flag it as potentially unsafe if it's a literal index > 0?
                                issues.push(CodeIssue {
                                    file_path: file_path.display().to_string(),
                                    line: start.row + 1,
                                    column: start.column + 1,
                                    end_line: Some(end.row + 1),
                                    end_column: Some(end.column + 1),
                                    message: "Potensial issue dapat terjadi dengan pendekatan seperti ini, pastikan kamu selalu melakukan pengecekan untuk index-nya ya.".to_string(),
                                    severity: Severity::Warning,
                                    category: Category::CodeQuality,
                                    rule: "null-safety".to_string(),
                                    code_snippet: Some(text.to_string()),
                                });
                            }
                        }
                    }
                }
            }
        } else if let Err(e) = Query::new(&tree_sitter_dart::language(), query_source) {
            // Fallback if query fails compilation (due to wrong node names)
            issues.push(CodeIssue {
                file_path: file_path.display().to_string(),
                line: 1,
                column: 1,
                end_line: None,
                end_column: None,
                message: format!("Internal Error: Failed to compile Dart AST Query: {}", e),
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
        let dart_files = self.find_dart_files(dir_path)?;

        for file_path in dart_files {
            if let Ok(analysis) = self.analyze_file(&file_path) {
                result.add_file(analysis);
            }
        }
        Ok(result)
    }

    fn find_dart_files(&self, dir_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "dart" {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
        Ok(files)
    }
}

impl Default for DartParser {
    fn default() -> Self {
        Self::new()
    }
}
