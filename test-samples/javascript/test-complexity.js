// Test file for complexity analysis

// Simple function - should be OK
function simpleFunction(a, b) {
    return a + b;
}

// Function with high complexity
function complexFunction(x, y, z) {
    if (x > 0) {
        if (y > 0) {
            if (z > 0) {
                if (x > y) {
                    if (y > z) {
                        return x;
                    }
                }
            }
        }
    }
    return 0;
}

// Function with many parameters (should warn)
function tooManyParameters(a, b, c, d, e, f) {
    return a + b + c + d + e + f;
}

// Function with many branching paths
function processRequest(data) {
    if (!data) {
        return null;
    }

    if (data.type === 'user') {
        if (data.role === 'admin') {
            if (data.active) {
                return { allowed: true };
            } else {
                return { allowed: false, reason: 'inactive' };
            }
        } else if (data.role === 'user') {
            return { allowed: true, limited: true };
        }
    } else if (data.type === 'guest') {
        return { allowed: false };
    }

    return null;
}

// Large block with many statements
function largeFunction() {
    let result = [];
    for (let i = 0; i < 10; i++) {
        result.push(i);
    }
    // ... many more statements
    return result;
}
