// Test file for unused variables and code

void main() {
  // Unused variables
  int unusedVariable = 42;
  String unusedString = "never used";
  double unusedDouble = 3.14;
  
  // Used variable (should not warn)
  int usedVariable = 10;
  print(usedVariable);
  
  // Unused function parameter
  calculateSum(5, 10);
  
  // Unused imports would be detected here
  var list = [1, 2, 3];
  
  // Unused local variable in block
  {
    int blockVar = 100;
    int usedBlockVar = 200;
    print(usedBlockVar);
  }
}

void calculateSum(int a, int b) {
  // Parameter 'b' is unused
  print(a);
}

class UnusedClass {
  int unusedField = 0;
  int usedField = 1;
  
  void unusedMethod() {
    print("never called");
  }
  
  void usedMethod() {
    print(usedField);
  }
}

void testFunction() {
  var obj = UnusedClass();
  obj.usedMethod();
}
