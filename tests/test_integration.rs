use js_ast_analyzer::JsParser;
use std::path::PathBuf;

#[test]
fn test_analyze_all_test_samples() {
    let parser = JsParser::new();
    let test_dir = PathBuf::from("test-samples");
    
    let result = parser.analyze_directory(&test_dir);
    assert!(result.is_ok(), "Failed to analyze test-samples directory");
    
    let analysis = result.unwrap();
    
    // Should analyze multiple files
    assert!(analysis.files.len() > 0, "Should analyze at least one file");
    
    // Should have some issues across all files
    assert!(analysis.summary.total > 0, "Should detect issues across test files");
}

#[test]
fn test_typescript_file() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/sample.ts");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse TypeScript file");
    
    let analysis = result.unwrap();
    
    // TypeScript files should be parsed successfully
    assert!(analysis.file_path.ends_with("sample.ts"));
}

#[test]
fn test_javascript_file() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/sample.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse JavaScript file");
    
    let analysis = result.unwrap();
    
    // JavaScript files should be parsed successfully
    assert!(analysis.file_path.ends_with("sample.js"));
}

#[test]
fn test_summary_consistency() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-naming.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Summary totals should match
    let calculated_total = analysis.summary.error + analysis.summary.warning + analysis.summary.suggestion;
    assert_eq!(calculated_total, analysis.summary.total, "Summary total should match sum of severities");
    assert_eq!(analysis.summary.total, analysis.issues.len(), "Summary total should match issue count");
}

#[test]
fn test_invalid_file() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("non-existent-file.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_err(), "Should fail on non-existent file");
}
