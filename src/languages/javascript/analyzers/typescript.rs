use super::Analyzer;
use crate::types::{CodeIssue, Category, Severity};
use oxc_ast::ast::*;
use oxc_span::Span;
use std::path::Path;

pub struct TypeScriptAnalyzer;

impl TypeScriptAnalyzer {
    pub fn new() -> Self {
        Self
    }

    fn get_line_column(source_code: &str, span: Span) -> (usize, usize) {
        let start = span.start as usize;
        let before = &source_code[..start];
        let line = before.lines().count();
        let last_newline = before.rfind('\n').unwrap_or(0);
        let column = start - last_newline;
        (line, column)
    }

    fn add_issue(
        &self,
        issues: &mut Vec<CodeIssue>,
        file_path: &Path,
        source_code: &str,
        span: Span,
        message: String,
        rule: String,
        severity: Severity,
    ) {
        let (line, column) = Self::get_line_column(source_code, span);
        let start = span.start as usize;
        let end = span.end as usize;
        let code_snippet = source_code.get(start..end).map(|s| s.to_string());

        issues.push(CodeIssue {
            file_path: file_path.display().to_string(),
            line,
            column,
            end_line: None,
            end_column: None,
            message,
            severity,
            category: Category::TypeScript,
            rule,
            code_snippet,
        });
    }
}

impl Analyzer for TypeScriptAnalyzer {
    fn analyze(&self, program: &Program, file_path: &Path, source_code: &str) -> Vec<CodeIssue> {
        let mut issues = Vec::new();

        // Only check TypeScript files
        if !file_path.to_string_lossy().ends_with(".ts")
            && !file_path.to_string_lossy().ends_with(".tsx")
        {
            return issues;
        }

        for stmt in &program.body {
            self.analyze_statement(&mut issues, stmt, file_path, source_code);
        }

        issues
    }
}

impl TypeScriptAnalyzer {
    fn analyze_statement(
        &self,
        issues: &mut Vec<CodeIssue>,
        stmt: &Statement,
        file_path: &Path,
        source_code: &str,
    ) {
        match stmt {
            Statement::VariableDeclaration(var_decl) => {
                for var in &var_decl.declarations {
                    if let Some(type_ann) = &var.id.type_annotation {
                        self.analyze_ts_type(issues, &type_ann.type_annotation, file_path, source_code);
                    }
                }
            }
            Statement::FunctionDeclaration(func) => {
                // Check for missing return type on exported functions
                if func.return_type.is_none() {
                    self.add_issue(
                        issues,
                        file_path,
                        source_code,
                        func.span,
                        "Tipe return hilang pada fungsi - tambahkan tipe return eksplisit untuk keamanan tipe yang lebih baik".to_string(),
                        "explicit-function-return-type".to_string(),
                        Severity::Suggestion,
                    );
                } else if let Some(return_type) = &func.return_type {
                    self.analyze_ts_type(issues, &return_type.type_annotation, file_path, source_code);
                }
            }
            _ => {}
        }
    }

    fn analyze_ts_type(
        &self,
        issues: &mut Vec<CodeIssue>,
        ts_type: &TSType,
        file_path: &Path,
        source_code: &str,
    ) {
        match ts_type {
            TSType::TSAnyKeyword(any_type) => {
                self.add_issue(
                    issues,
                    file_path,
                    source_code,
                    any_type.span,
                    "Hindari penggunaan tipe 'any' - ini menghilangkan manfaat TypeScript".to_string(),
                    "no-any-type".to_string(),
                    Severity::Suggestion,
                );
            }
            TSType::TSArrayType(array_type) => {
                // Recursively check element type
                self.analyze_ts_type(issues, &array_type.element_type, file_path, source_code);
            }
            TSType::TSUnionType(union_type) => {
                // Check all types in union
                for type_ann in &union_type.types {
                    self.analyze_ts_type(issues, type_ann, file_path, source_code);
                }
            }
            _ => {}
        }
    }
}
