// Test file for naming conventions

void main() {
  // Generic names
  var data = "some data";
  var result = 42;
  var info = "information";
  var value = 100;
  
  // Short names (should be flagged except loop counters)
  var x = 10;
  var y = 20;
  var a = "test";
  
  // Loop counters (should be allowed)
  for (int i = 0; i < 10; i++) {
    for (int j = 0; j < 5; j++) {
      print(i * j);
    }
  }
  
  // Boolean without proper prefix
  bool active = true;
  bool enabled = false;
  bool valid = true;
  
  // Good boolean names
  bool isActive = true;
  bool hasPermission = false;
  bool canEdit = true;
  bool shouldUpdate = false;
}

// Bad class name
class myclass {
  void doSomething() {}
}

// Good class name
class UserProfile {
  void updateProfile() {}
}

// Bad function name
void Do_Something() {
  print("bad naming");
}

// Good function name
void calculateTotal() {
  print("good naming");
}
