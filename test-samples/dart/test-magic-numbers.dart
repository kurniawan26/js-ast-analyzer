// Test file for magic numbers and hardcoded values

void main() {
  // Magic numbers (should be constants)
  double pi = 3.14159;
  double taxRate = 0.15;
  int timeout = 5000;
  int maxRetries = 3;
  
  // Allowed numbers (0, 1, 2, 10, 100)
  int zero = 0;
  int one = 1;
  int two = 2;
  int ten = 10;
  
  // Magic numbers in calculations
  double area = 5.5 * 10.2;
  int total = 42 + 17;
  
  // Magic numbers in conditions
  if (timeout > 3000) {
    print("timeout too long");
  }
  
  // Hardcoded strings (should be constants)
  String apiUrl = "https://api.example.com/v1/users";
  String errorMessage = "An error occurred while processing your request";
  String successMessage = "Operation completed successfully";
  
  // Good: Using constants
  const double PI = 3.14159;
  const int MAX_RETRIES = 3;
  const String API_URL = "https://api.example.com";
  
  print(PI);
  print(MAX_RETRIES);
  print(API_URL);
}

class Configuration {
  // Magic numbers in class
  int maxConnections = 100;
  double connectionTimeout = 30.5;
  String defaultLanguage = "en-US";
  
  // Good: Using constants
  static const int MAX_CONNECTIONS = 100;
  static const double CONNECTION_TIMEOUT = 30.5;
}

void calculatePrice() {
  // Magic numbers in function
  double price = 99.99;
  double discount = 0.2;
  double tax = 0.08;
  
  double finalPrice = price * (1 - discount) * (1 + tax);
  print(finalPrice);
}
