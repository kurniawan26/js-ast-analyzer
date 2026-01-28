def add_numbers(a, b):
    # This is a good function
    return a + b

class HelperParams:
    def __init__(self, value):
        self.value = value

CONSTANT_VAL = 100

def process_data(data):
    if data:
        print("Data is valid")  # Allowed if no strict rule, but we check for print use logger
