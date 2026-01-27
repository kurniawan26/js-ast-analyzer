// Rule: class-naming, variable-naming

class bad_class_name { // Should warn: class-naming
}

class GoodClassName {
    fun method() {
        val BadVar = 1 // Should warn: variable-naming
        val goodVar = 2
    }
}
