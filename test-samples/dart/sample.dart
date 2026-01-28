void main() {
    print("Hello World"); // Should strict warning

    int magic = 42; // Should trigger magic number

    if (true) {
        if (true) {
             print("Too deep"); // Nested if
        }
    }

    String variable_name = "ok";
    String BadName = "bad"; // Variable naming

    // Unused
    int unusedVar = 0;
}

class badClassName { // Class naming
}

class GoodClassName {
}
