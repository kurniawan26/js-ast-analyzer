# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based JavaScript/TypeScript AST analyzer that uses the OXC parser to detect code quality, security, and best practice issues. It's designed as a standalone binary for integration with autograder systems and CI/CD pipelines.

**Key characteristic**: Uses proper AST parsing via OXC (not regex) for accurate code analysis.

## Build and Run Commands

```bash
# Development build with debug output
cargo build

# Release build (optimized, stripped binary)
cargo build --release

# Run analyzer on a file or directory
./target/release/js-ast-analyzer <path>

# Output JSON format (for CI/CD integration)
./target/release/js-ast-analyzer <path> --format json

# Strict mode (exit with error code if issues found)
./target/release/js-ast-analyzer <path> --strict

# Run tests
cargo test

# Run specific test
cargo test --test <test_name>

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

The release binary is located at `target/release/js-ast-analyzer`.

## Architecture

### Core Components

**Entry Point**: `src/main.rs`
- Parses CLI arguments using `clap`
- Validates input path
- Creates `JsParser` and runs analysis
- Formats and outputs results

**Parser Layer**: `src/parser.rs`
- `JsParser` struct wraps OXC parser and coordinates analyzers
- `analyze_file()`: Parses single file, runs all analyzers
- `analyze_directory()`: Recursively finds .js/.jsx/.ts/.tsx files (skips node_modules)
- Returns `FileAnalysis` or `AnalysisResult`

**Analyzer Framework**: `src/analyzers/mod.rs`
- `Analyzer` trait: `analyze(&self, program: &Program, file_path: &Path, source_code: &str) -> Vec<CodeIssue>`
- `Analyzers` struct: Collection of all analyzer implementations
- All analyzers run on each file, results combined

**Individual Analyzers**: `src/analyzers/*.rs`
- `security.rs`: Detects `eval()`, `innerHTML`, `alert()`, console methods
- `typescript.rs`: Detects `any` types (only runs on .ts/.tsx files)
- `best_practices.rs`: Detects var usage, equality operators
- `patterns.rs`: Pattern-based checks

**Type System**: `src/types.rs`
- `CodeIssue`: Issue with file path, line/column, message, severity, category, rule ID, snippet
- `Severity`: Error, Warning, Suggestion
- `Category`: Security, BestPractice, CodeQuality, Performance, Maintainability, TypeScript
- `FileAnalysis`: Issues + summary for one file
- `AnalysisResult`: Collection of file analyses + total summary

**Output Formatting**: `src/output/mod.rs`
- Human-readable format with color coding (terminal output)
- JSON format for programmatic consumption

**Error Handling**: `src/error.rs`
- `AnalyzerError` enum using `thiserror`
- Covers parse errors, file I/O, invalid paths

### AST Traversal Pattern

Analyzers manually traverse the OXC AST. Each analyzer implements recursive visitor methods:

```rust
fn analyze_statement(&self, issues: &mut Vec<CodeIssue>, stmt: &Statement, ...) {
    match stmt {
        Statement::ExpressionStatement(expr) => { /* handle */ }
        Statement::BlockStatement(block) => {
            for stmt in &block.body {
                self.analyze_statement(issues, stmt, ...);
            }
        }
        // ... other statement types
    }
}
```

Important: Must traverse into nested structures (blocks, functions, loops) to find all issues.

### Span-to-Line Conversion

Helper method in each analyzer converts OXC `Span` (byte offsets) to line/column numbers:

```rust
fn get_line_column(source_code: &str, span: Span) -> (usize, usize) {
    let start = span.start as usize;
    let before = &source_code[..start];
    let line = before.lines().count();
    let last_newline = before.rfind('\n').unwrap_or(0);
    let column = start - last_newline;
    (line, column)
}
```

This is critical for accurate error reporting.

## Adding New Analysis Rules

1. **Choose appropriate analyzer file** (security.rs, typescript.rs, best_practices.rs) or create new one
2. **Add detection logic** in AST traversal methods:
   - Match on specific AST node types
   - Extract `Span` for location
   - Call `add_issue()` with details
3. **Register analyzer** in `src/analyzers/mod.rs` if creating new file:
   ```rust
   pub struct Analyzers {
       pub your_analyzer: your_module::YourAnalyzer,
   }
   ```
4. **Add to `analyze_module()`** in `src/analyzers/mod.rs`

**Example**: Adding a check for `document.write()`:

```rust
// In security.rs or similar
Expression::CallExpression(call_expr) => {
    if let Expression::StaticMemberExpression(member) = &call_expr.callee {
        if let Expression::Identifier(ident) = &member.object {
            if ident.name == "document" && member.property.name == "write" {
                self.add_issue(issues, file_path, source_code, call_expr.span,
                    "Avoid document.write() - it clears the document".to_string(),
                    "no-document-write".to_string());
            }
        }
    }
}
```

## Integration Points

The binary is designed for external integration:

- **JSON output**: `--format json` returns structured `AnalysisResult`
- **Exit codes**: `--strict` exits with code 1 if issues found (for CI/CD)
- **Stdin support**: Currently not implemented, could be added

See `INTEGRATION.md` for Node.js integration examples with autograder systems.

## Dependencies

- **oxc_parser**: Fast JS/TS parser (main dependency)
- **oxc_ast**: AST node types
- **clap**: CLI argument parsing
- **serde/serde_json**: Serialization for JSON output
- **walkdir**: Directory traversal
- **anyhow/thiserror**: Error handling

## Known Limitations

- Manual AST traversal (no visitor pattern framework) - could miss cases if traversal incomplete
- No configuration file support (yet)
- No ignore comments support (yet) - e.g., `// analyzer-disable-next-line`
- TypeScript analyzer only runs on files with .ts/.tsx extension

## Testing

Test files located in `test-samples/` directory. Run the binary on sample files to verify behavior.
