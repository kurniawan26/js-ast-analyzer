// Test file for unused variable detection

const unusedVar = "this is never used";
let anotherUnused = 123;

function usedFunction() {
    return "used";
}

function unusedFunction() {
    return "not called anywhere";
}

const used = usedFunction();
console.log(used);

// Variable with underscore prefix should be ignored
const _ignoreThis = "should not trigger warning";

// Used variable
const active = true;
if (active) {
    console.log("active is true");
}
