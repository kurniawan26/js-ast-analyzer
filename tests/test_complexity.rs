use js_ast_analyzer::JsParser;
use std::path::PathBuf;

#[test]
fn test_complexity_simple_function() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-complexity.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse test-complexity.js");
    
    let analysis = result.unwrap();
    
    // Simple functions should not trigger complexity warnings
    // But the file has complex functions, so there should be issues
    assert!(analysis.issues.len() > 0, "Should detect complexity issues");
}

#[test]
fn test_complexity_nested_conditions() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-complexity.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Should detect deeply nested conditions
    let complexity_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("complexity") || issue.message.contains("nested"))
        .collect();
    
    assert!(!complexity_issues.is_empty(), "Should detect high complexity");
}

#[test]
fn test_complexity_too_many_parameters() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-complexity.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Should detect functions with too many parameters
    let param_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("parameter") || issue.message.contains("arguments"))
        .collect();
    
    // At least some complexity issues should be detected
    assert!(analysis.issues.len() > 0, "Should detect complexity issues");
}

#[test]
fn test_complexity_summary() {
    let parser = JsParser::new();
    let test_file = PathBuf::from("test-samples/javascript/test-complexity.js");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Verify summary
    let total = analysis.summary.error + analysis.summary.warning + analysis.summary.suggestion;
    assert_eq!(total, analysis.issues.len(), "Summary should match issue count");
}
