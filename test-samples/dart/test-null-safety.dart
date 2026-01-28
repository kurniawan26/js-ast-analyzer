// Test file for null safety issues

void main() {
  // Unsafe property access
  Map<String, dynamic>? user;
  print(user['name']); // Should warn about null check
  
  // Chained property access
  var obj = getObject();
  print(obj.property.subProperty); // Unsafe chaining
  
  // Array access without bounds check
  List<int> numbers = [1, 2, 3];
  print(numbers[10]); // Unsafe array access
  
  // Null check examples
  String? nullableString;
  print(nullableString.length); // Should warn
  
  // Safe access (should not warn)
  String? safeString;
  if (safeString != null) {
    print(safeString.length);
  }
  
  // Using null-aware operators (good)
  print(safeString?.length);
  print(safeString ?? "default");
}

dynamic getObject() {
  return null;
}

class Person {
  String? name;
  Address? address;
}

class Address {
  String? street;
  String? city;
}

void unsafeAccess() {
  Person? person;
  // Unsafe nested access
  print(person.address.street);
}
