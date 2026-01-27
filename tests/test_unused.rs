use js_ast_analyzer::JsParser;
use std::path::PathBuf;

#[test]
fn test_unused_variables() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-unused.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse test-unused.js");
    
    let analysis = result.unwrap();
    
    // Should detect unused variables
    let unused_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("unused") || issue.message.contains("declared but never used"))
        .collect();
    
    assert!(!unused_issues.is_empty(), "Should detect unused variables");
}

#[test]
fn test_unused_summary() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-unused.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Verify summary
    let total = analysis.summary.error + analysis.summary.warning + analysis.summary.suggestion;
    assert_eq!(total, analysis.issues.len(), "Summary should match issue count");
}
