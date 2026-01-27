// Rule: nested-if

fun complexLogic(x: Int) {
    if (x > 0) {
        if (x < 100) {
             if (x == 50) { // Should warn: nested-if (depth 2)
                 println("Halfway")
             }
        }
    }
}
