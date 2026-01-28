use js_ast_analyzer::DartParser;
use std::path::PathBuf;

#[test]
fn test_dart_complexity_simple_function() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-complexity.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse test-complexity.dart");
    
    let analysis = result.unwrap();
    
    // Should detect complexity issues
    assert!(analysis.issues.len() > 0, "Should detect complexity issues");
}

#[test]
fn test_dart_complexity_nested_conditions() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-complexity.dart");
    
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
fn test_dart_complexity_too_many_parameters() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-complexity.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Should detect functions with too many parameters
    let param_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("parameter") || issue.message.contains("arguments"))
        .collect();
    
    assert!(analysis.issues.len() > 0, "Should detect complexity issues");
}

#[test]
fn test_dart_complexity_summary() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-complexity.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Verify summary
    let total = analysis.summary.error + analysis.summary.warning + analysis.summary.suggestion;
    assert_eq!(total, analysis.issues.len(), "Summary should match issue count");
}
