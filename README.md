# Multi-Language AST Analyzer

A fast, standalone Rust binary for analyzing code quality, security issues, and best practices across multiple programming languages using modern AST parsers (**Oxc** for JS/TS, **Tree-sitter** for others).

## Features

- **Multi-Language Support**:
  - **JavaScript / TypeScript** (via `oxc`)
  - **Kotlin** (via `tree-sitter-kotlin`)
  - **Dart** (via `tree-sitter-dart`)
  - **Python** (via `tree-sitter-python`)
- **Fast Analysis**: Written in Rust for maximum performance.
- **AST-Based Precision**: Uses real abstract syntax trees instead of fragile regex matching.
- **Multiple Output Formats**: Human-readable console output or JSON for automation.
- **Standalone**: No compiler or runtime dependencies required for analysis.

## implementation Status

| Language | Parser | Key Features |
|----------|--------|--------------|
| **JavaScript / TypeScript** | [Oxc](https://github.com/oxc-project/oxc) | Security (XSS, eval), Best Practices, Complexity, Types |
| **Kotlin** | [Tree-sitter](https://tree-sitter.github.io/) | Naming conventions, Magic numbers, Nested expressions, Unused vars |
| **Dart** | [Tree-sitter](https://tree-sitter.github.io/) | Null safety checks, Naming conventions, Complexity |
| **Python** | [Tree-sitter](https://tree-sitter.github.io/) | Naming conventions (snake_case), Magic numbers, Complexity |

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd js-ast-analyzer

# Build release binary
cargo build --release

# The binary will be at target/release/js-ast-analyzer
```

## Usage

Basic syntax:
```bash
js-ast-analyzer [OPTIONS] <PATH>
```

### Examples

**Analyze JavaScript/TypeScript (Default):**
```bash
./target/release/js-ast-analyzer src/
```

**Analyze Kotlin Code:**
```bash
./target/release/js-ast-analyzer src/main/kotlin --language kotlin
```

**Analyze Dart Code:**
```bash
./target/release/js-ast-analyzer lib/ --language dart
```

**Analyze Python Code:**
```bash
./target/release/js-ast-analyzer src/ --language python
```

**Output in JSON (for CI/CD):**
```bash
./target/release/js-ast-analyzer src/ -f json
```

### Command-Line Options

```
USAGE:
    js-ast-analyzer [OPTIONS] <PATH>

ARGUMENTS:
    <PATH>    Path to file or directory to analyze

OPTIONS:
    -l, --language <LANGUAGE>   Language to analyze [default: javascript]
                                [possible values: javascript, typescript, kotlin, dart, python]
    -f, --format <FORMAT>       Output format [default: human] [possible values: json, human]
    -s, --strict                Exit with error code if any issues are found
    -h, --help                  Print help information
    -V, --version               Print version information
```

## Supported Rules by Language

### JavaScript / TypeScript
- **Security**: `eval()`, `innerHTML`, `document.write()`, hardcoded secrets.
- **Best Practices**: `var` usage, `==` vs `===`, `console.log` in production.
- **Code Quality**: Magic numbers, long strings, maintainability metrics.

### Kotlin
- **Naming**: Class (PascalCase), Function/Variable (camelCase).
- **Complexity**: Nested `if` statements (>2 levels), Too many parameters (>5).
- **Best Practices**: `no-print`, `no-magic-numbers`, `unused-variable`.

### Dart
- **Null Safety**: Unsafe property access (`.`) on nullable types (suggests `?.`), Unsafe array access.
- **Naming**: Class (PascalCase), Function/Variable (camelCase), Boolean prefixes (`is`, `has`, `should`).
- **Complexity**: Deep nesting, function length/params.

### Python
- **Naming**: Function/Variable (snake_case), Class (PascalCase), Constants (UPPER_CASE).
- **Best Practices**: `no-print`, `no-magic-numbers` (except 0, 1, -1, etc.).
- **Complexity**: Deep nesting, parameter count.

## output Examples

### Human-Readable

```
═══════════════════════════════════════
  Code Quality Analysis Results
═══════════════════════════════════════

Total Files: 1
Total Issues: 3 (0 errors, 2 warnings, 1 suggestion)

src/sample.py:
────────────────────────────────────────────────────────────────────────────────
  ⚠ 4:5:warning
    Avoid using print() in production. Use a logger.
    [rule: no-print]
    > print("Data")

  ⚠ 10:1:warning
    Variable name 'myData' should be snake_case.
    [rule: variable-naming]
    > myData

  ℹ 15:12:suggestion
    magic number detected: 42. Define a constant.
    [rule: no-magic-numbers]
    > 42
```

### JSON

```json
{
  "files": [
    {
      "file_path": "src/sample.py",
      "issues": [
        {
          "file_path": "src/sample.py",
          "line": 4,
          "column": 5,
          "message": "Avoid using print() in production. Use a logger.",
          "severity": "warning",
          "category": "best-practice",
          "rule": "no-print",
          "code_snippet": "print(\"Data\")"
        }
      ],
      "summary": {
        "error": 0,
        "warning": 1,
        "suggestion": 0,
        "total": 1
      }
    }
  ],
  "summary": { "total": 1 }
}
```


## Integration with Autograder

This tool is designed to integrate seamlessly with autograder systems:

### Node.js Integration

```typescript
import { spawn } from 'child_process';

async function analyzeCode(projectPath: string) {
  const analyzerPath = '/path/to/js-ast-analyzer';
  const result = await spawn(analyzerPath, [projectPath, '--language', 'javascript', '--format', 'json']);

  const analysisResult = JSON.parse(result.stdout);
  return analysisResult;
}

// Use in your review process
const analysis = await analyzeCode(submissionPath);

// Generate informative feedback
if (analysis.summary.total > 0) {
  const feedback = generateFeedback(analysis);
  // Include in your report
}
```

### Example Feedback Generator

```typescript
function generateCodeQualityFeedback(result: AnalysisResult): string | null {
  if (result.summary.total === 0) return null;

  let feedback = '## Code Quality Suggestions\n\n';

  for (const file of result.files) {
    if (file.issues.length > 0) {
      feedback += `### ${file.file_path}\n\n`;
      for (const issue of file.issues) {
        feedback += `- Line ${issue.line}: ${issue.message}\n`;
      }
      feedback += '\n';
    }
  }

  return feedback;
}
```

## Contributing

We use a modular architecture located in `src/languages/`. To add a new language:

1. Add the Tree-sitter grammar dependency to `Cargo.toml`.
2. Create `src/languages/<language>/parser.rs`.
3. Implement the `analyze_file` function using Tree-sitter queries.
4. Register the module in `src/languages/mod.rs` and update `main.rs`.