// Test file for magic numbers and hardcoded values

// Magic numbers - should trigger warnings
function calculateArea(radius) {
    return 3.14159 * radius * radius; // 3.14159 is a magic number
}

function calculatePrice(amount) {
    return amount * 0.1; // 0.1 is a magic number (tax rate)
}

function setTimer() {
    setTimeout(callback, 5000); // 5000 is a magic number
}

function getPageLimit() {
    return 25; // 25 is a magic number
}

// Allowed numbers - should NOT trigger
function simpleMath(a) {
    return a * 2; // 2 is allowed (power of 2)
}

function basicCounter() {
    let count = 0; // 0 is allowed
    for (let i = 0; i < 10; i++) { // 10 is allowed
        count += 1; // 1 is allowed
    }
    return count;
}

// Long hardcoded string - should trigger
function getErrorMessage() {
    return "This is a very long error message that should be in a constant file instead of being hardcoded";
}

function processUrl() {
    return "https://api.example.com/v1/users/profile"; // Long hardcoded URL
}

// Short strings - should NOT trigger
function getStatus() {
    return "active"; // Short, acceptable
}

function getCode() {
    return "OK"; // Very short, acceptable
}
