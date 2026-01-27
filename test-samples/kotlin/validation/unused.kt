// Rule: unused-variable

fun unused() {
    val usedVar = 10
    println(usedVar)
    
    val unusedVar = 20; // Should warn: unused-variable
}
