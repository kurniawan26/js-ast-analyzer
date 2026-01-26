// Sample JavaScript file with various code quality issues

// Issue: Using var instead of let/const
var oldStyle = "should use let or const";
var another = true;

// Issue: Using == instead of ===
if (oldStyle == "test") {
    console.log("loose equality");
}

// Issue: console.log in production code
console.log("Debugging output");
console.error("Error logging");

// Issue: eval() usage - security risk
function dangerous(userInput) {
    eval(userInput); // DANGEROUS!
}

// Issue: innerHTML usage - XSS risk
function updateContent(userContent) {
    document.getElementById('content').innerHTML = userContent;
}

// Issue: alert() usage
alert("Hello!");

// Issue: debugger statement
debugger;

// Good practices
const goodConst = "this is fine";
let goodLet = "also fine";

if (goodConst === "test") { // strict equality
    // no console output
}

// Proper DOM manipulation
function safeUpdate(content) {
    document.getElementById('content').textContent = content;
}

// Testing recursion in control structures
// These should ideally be caught, but current BestPracticeAnalyzer might miss them if not recursive
if (true) {
    var nestedVar = "should be caught"; // Nested var
    if (nestedVar == "test") { // Nested loose equality
        console.log("nested equality check");
    } else {
        var elseVar = "in else block"; // Var in else
    }
}

for (var i = 0; i < 10; i++) { // Var in for loop init
    if (i == 5) { // Loose equality in loop
        var loopVar = "inside loop"; // Var inside loop
    }
}
