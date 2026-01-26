// Test file for new detection rules

// Security: new Function constructor
const dangerousFunc = new Function('return 42');

// Security: setTimeout with string
setTimeout('alert("hello")', 1000);

// Security: setInterval with string
setInterval('console.log("tick")', 1000);

// Security: document.write
document.write('<p>Hello</p>');

// Security: outerHTML
element.outerHTML = '<div>replacement</div>';

// Best practices: empty catch block
try {
    riskyOperation();
} catch (error) {
    // Empty catch - nothing here
}

// Best practices: double negation
const isActive = !!value;

// Best practices: void operator
const result = void 0;

// Best practices: comma operator
const x = (1, 2, 3);

// Best practices: debugger statement
debugger;

// Code quality: console methods
console.log('This should be removed');
console.debug('Debug info');
console.info('Info message');
console.warn('Warning message');
console.error('Error message');

// Security: Hardcoded secrets
const myPassword = "superSecretPassword123";
const apiKey = "12345-abcde-67890";
const api_token = "abcdef123456";
