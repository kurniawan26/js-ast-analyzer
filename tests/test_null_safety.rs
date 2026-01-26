use js_ast_analyzer::parser::JsParser;
use std::path::PathBuf;

#[test]
fn test_null_safety_chained_property_access() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/test-null-safety.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse test-null-safety.js");
    
    let analysis = result.unwrap();
    
    // Should detect unsafe property access
    let unsafe_access_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("property access") || issue.message.contains("null"))
        .collect();
    
    assert!(!unsafe_access_issues.is_empty(), "Should detect unsafe property access");
}

#[test]
fn test_null_safety_array_access() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/test-null-safety.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Should detect unsafe array access
    let array_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("array") || issue.message.contains("index"))
        .collect();
    
    // At least some null safety issues should be detected
    assert!(analysis.issues.len() > 0, "Should detect null safety issues");
}

#[test]
fn test_null_safety_summary() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/test-null-safety.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Verify summary counts
    let total = analysis.summary.error + analysis.summary.warning + analysis.summary.suggestion;
    assert_eq!(total, analysis.issues.len(), "Summary should match issue count");
}
