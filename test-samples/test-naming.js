// Test file for naming convention analysis

// Generic names - should trigger warnings
const data = fetch('/api/data'); // 'data' is too generic
const result = processData(data); // 'result' is too generic
const info = { id: 1, name: 'test' }; // 'info' is too generic
const value = 42; // 'value' is too generic
const item = items[0]; // 'item' is too generic
const obj = {}; // 'obj' is too generic

// Short names - should trigger (except loop counters)
const a = 1; // Too short
const b = 2; // Too short
const ab = 3; // Should be OK (3 chars)

// Boolean without prefix - should trigger
const active = true; // Should be isActive
const visible = false; // Should be isVisible
const enabled = true; // Should be isEnabled

// Good boolean names - should NOT trigger
const isActive = true;
const hasPermission = false;
const canEdit = true;
const shouldUpdate = false;

// Generic function names - should trigger
function handle(data) { // 'handle' is too generic
    return data;
}

function process(data) { // 'process' is too generic
    return data;
}

function execute(cmd) { // 'execute' is too generic
    return cmd;
}

// Good function names - should NOT trigger
function handleUserLogin(user) {
    return user;
}

function processPaymentData(data) {
    return data;
}

// Generic parameter names - should trigger
function calculate(data, opts) { // Both parameters are generic
    return data.value;
}

// Good parameter names - should NOT trigger
function calculatePrice(quantity, taxRate) {
    return quantity * taxRate;
}

// Loop counters - should NOT trigger
for (let i = 0; i < 10; i++) {
    console.log(i);
}

for (let j = 0; j < 10; j++) {
    for (let k = 0; k < 10; k++) {
        console.log(j, k);
    }
}

// Generic names in different contexts
const temp = getTemp(); // 'temp' is generic
const arr = [1, 2, 3]; // 'arr' is generic
const str = "hello"; // 'str' is generic
const num = 42; // 'num' is generic
