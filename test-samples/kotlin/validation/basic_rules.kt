// Rule: no-print, no-magic-numbers

fun logging() {
    println("This is a log") // Should warn: no-print
    print("Another log")     // Should warn: no-print
}

fun maths() {
   val x = 100 // Should warn: no-magic-numbers (100)
}
