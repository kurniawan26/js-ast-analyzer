fun calculate(a: Int, b: Int): Int {
    return a + b
}

class User(val name: String) {
    fun greet() {
        // No println here, should be clean
        val message = "Hello $name"
    }
}
