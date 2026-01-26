pub mod patterns;
pub mod typescript;
pub mod security;
pub mod best_practices;
pub mod unused;
pub mod complexity;
pub mod magic_numbers;
pub mod naming;
pub mod null_safety;

use crate::types::CodeIssue;
use oxc_ast::ast::Program;
use std::path::Path;

/// Trait for AST analyzers
pub trait Analyzer {
    /// Analyze a module and return any issues found
    fn analyze(&self, program: &Program, file_path: &Path, source_code: &str) -> Vec<CodeIssue>;
}

/// Collection of all analyzers
pub struct Analyzers {
    pub patterns: patterns::PatternAnalyzer,
    pub typescript: typescript::TypeScriptAnalyzer,
    pub security: security::SecurityAnalyzer,
    pub best_practices: best_practices::BestPracticeAnalyzer,
    pub unused: unused::UnusedAnalyzer,
    pub complexity: complexity::ComplexityAnalyzer,
    pub magic_numbers: magic_numbers::MagicNumberAnalyzer,
    pub naming: naming::NamingAnalyzer,
    pub null_safety: null_safety::NullSafetyAnalyzer,
}

impl Analyzers {
    pub fn new() -> Self {
        Self {
            patterns: patterns::PatternAnalyzer::new(),
            typescript: typescript::TypeScriptAnalyzer::new(),
            security: security::SecurityAnalyzer::new(),
            best_practices: best_practices::BestPracticeAnalyzer::new(),
            unused: unused::UnusedAnalyzer::new(),
            complexity: complexity::ComplexityAnalyzer::new(),
            magic_numbers: magic_numbers::MagicNumberAnalyzer::new(),
            naming: naming::NamingAnalyzer::new(),
            null_safety: null_safety::NullSafetyAnalyzer::new(),
        }
    }

    pub fn analyze_module(&self, program: &Program, file_path: &Path, source_code: &str) -> Vec<CodeIssue> {
        let mut issues = Vec::new();

        issues.extend(self.patterns.analyze(program, file_path, source_code));
        issues.extend(self.typescript.analyze(program, file_path, source_code));
        issues.extend(self.security.analyze(program, file_path, source_code));
        issues.extend(self.best_practices.analyze(program, file_path, source_code));
        issues.extend(self.unused.analyze(program, file_path, source_code));
        issues.extend(self.complexity.analyze(program, file_path, source_code));
        issues.extend(self.magic_numbers.analyze(program, file_path, source_code));
        issues.extend(self.naming.analyze(program, file_path, source_code));
        issues.extend(self.null_safety.analyze(program, file_path, source_code));

        issues
    }
}

impl Default for Analyzers {
    fn default() -> Self {
        Self::new()
    }
}
