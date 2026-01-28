// Test file for complexity issues

void main() {
  complexFunction(1, 2, 3, 4, 5, 6, 7, 8);
  deeplyNestedFunction();
}

// Function with too many parameters
void complexFunction(int a, int b, int c, int d, int e, int f, int g, int h) {
  print(a + b + c + d + e + f + g + h);
}

// Deeply nested conditions
void deeplyNestedFunction() {
  int x = 10;
  
  if (x > 0) {
    if (x < 100) {
      if (x % 2 == 0) {
        if (x > 5) {
          if (x < 50) {
            print("Too deeply nested");
          }
        }
      }
    }
  }
}

// High cyclomatic complexity
void highComplexityFunction(int value) {
  if (value > 0) {
    print("positive");
  } else if (value < 0) {
    print("negative");
  } else {
    print("zero");
  }
  
  for (int i = 0; i < value; i++) {
    if (i % 2 == 0) {
      print("even");
    } else {
      print("odd");
    }
    
    if (i > 10) {
      break;
    }
  }
  
  switch (value) {
    case 1:
      print("one");
      break;
    case 2:
      print("two");
      break;
    case 3:
      print("three");
      break;
    default:
      print("other");
  }
}

// Long function (too many lines)
void longFunction() {
  print("line 1");
  print("line 2");
  print("line 3");
  print("line 4");
  print("line 5");
  print("line 6");
  print("line 7");
  print("line 8");
  print("line 9");
  print("line 10");
  print("line 11");
  print("line 12");
  print("line 13");
  print("line 14");
  print("line 15");
  print("line 16");
  print("line 17");
  print("line 18");
  print("line 19");
  print("line 20");
  print("line 21");
  print("line 22");
  print("line 23");
  print("line 24");
  print("line 25");
  print("line 26");
  print("line 27");
  print("line 28");
  print("line 29");
  print("line 30");
}
