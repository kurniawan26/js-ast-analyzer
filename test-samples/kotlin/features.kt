fun calculate(x: Int): Int {
    if (x > 10) {
        if (x > 20) {
            if (x > 30) {
                 return x * 2
            }
        }
    }
    return x
}

fun process(name: String?) {
    val len = name?.length // OK
    val unsafeLen = name!!.length // OK for now (not checking !!)
    val badNull = null
}

class badClassName {
}

val BadVariableName = "test"
val magic = 42
