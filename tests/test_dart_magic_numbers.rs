use js_ast_analyzer::DartParser;
use std::path::PathBuf;

#[test]
fn test_dart_magic_numbers_detection() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-magic-numbers.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse test-magic-numbers.dart");
    
    let analysis = result.unwrap();
    
    // Should detect magic numbers like 3.14159, 0.15, 5000
    let magic_number_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("magic number") || issue.message.contains("hardcoded"))
        .collect();
    
    assert!(!magic_number_issues.is_empty(), "Should detect magic numbers");
}

#[test]
fn test_dart_magic_numbers_allowed_values() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-magic-numbers.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Numbers like 0, 1, 2, 10 should be allowed
    let total_issues = analysis.issues.len();
    
    // Should have some issues but not flag every number
    assert!(total_issues > 0, "Should detect some magic numbers");
    assert!(total_issues < 50, "Should not flag allowed numbers excessively");
}

#[test]
fn test_dart_hardcoded_strings() {
    let parser = DartParser::new();
    let test_file = PathBuf::from("test-samples/dart/test-magic-numbers.dart");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    
    let analysis = result.unwrap();
    
    // Should detect long hardcoded strings
    let string_issues: Vec<_> = analysis.issues.iter()
        .filter(|issue| issue.message.contains("string") || issue.message.contains("hardcoded"))
        .collect();
    
    assert!(!string_issues.is_empty(), "Should detect hardcoded values");
}
