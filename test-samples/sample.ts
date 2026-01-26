// TypeScript sample with issues

// Issue: Using any type
function processData(data: any) {
    return data.value;
}

// Issue: Using @ts-ignore
// @ts-ignore
const problematic: any = "something";

// Issue: Using @ts-nocheck
// @ts-nocheck
function ignored() {
    return "this file's type checking is disabled";
}

// Good TypeScript
interface User {
    name: string;
    age: number;
}

function getUserData(user: User): string {
    return user.name;
}

// Generic type instead of any
function safeProcess<T>(data: T): T {
    return data;
}
