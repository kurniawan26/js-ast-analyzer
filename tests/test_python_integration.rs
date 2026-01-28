use js_ast_analyzer::{PythonParser, FileAnalysis};
use std::path::PathBuf;

#[test]
fn test_python_naming() {
    let parser = PythonParser::new();
    let test_file = PathBuf::from("test-samples/python/naming.py");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok(), "Failed to parse python file");
    
    let analysis = result.unwrap();
    
    let issues: Vec<_> = analysis.issues.iter()
        .map(|i| i.message.clone())
        .collect();

    // Check strict matching of issues
    assert!(issues.iter().any(|m| m.contains("Function name 'BadFunction' should be snake_case")));
    assert!(issues.iter().any(|m| m.contains("Class name 'bad_class' should be PascalCase")));
    assert!(issues.iter().any(|m| m.contains("Variable name 'variableName' should be snake_case")));
    // We didn't enable generic name check for Python yet in the code I wrote? 
    // Wait, I copied logic but did I include "generic names"? 
    // Looking at parser.rs code...
    // I only see check for snake_case in `var_assign`.
}

#[test]
fn test_python_complexity() {
    let parser = PythonParser::new();
    let test_file = PathBuf::from("test-samples/python/complexity.py");
    
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    let analysis = result.unwrap();
    let issues: Vec<_> = analysis.issues.iter().map(|i| i.message.clone()).collect();
    
    assert!(issues.iter().any(|m| m.contains("deeply nested if statements")));
    assert!(issues.iter().any(|m| m.contains("print()")));
    assert!(issues.iter().any(|m| m.contains("magic number detected: 42")));
    assert!(issues.iter().any(|m| m.contains("magic number detected: 3.14")));
    assert!(issues.iter().any(|m| m.contains("hardcoded string detected")));
}

#[test]
fn test_python_parameters() {
    let parser = PythonParser::new();
    let test_file = PathBuf::from("test-samples/python/naming.py"); // reused for params
    let result = parser.analyze_file(&test_file);
    assert!(result.is_ok());
    let analysis = result.unwrap();
    let issues: Vec<_> = analysis.issues.iter().map(|i| i.message.clone()).collect();
    
    assert!(issues.iter().any(|m| m.contains("Function has too many parameters (6)")));
}
