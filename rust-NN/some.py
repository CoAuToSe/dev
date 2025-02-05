class Array:
    def __init__(self, data):
        self.data = data

    def __add__(self, other):
        result = [a + b for a, b in zip(self.data, other.data)]
        return Array(result)

    def __sub__(self, other):
        result = [a - b for a, b in zip(self.data, other.data)]
        return Array(result)

    def __mul__(self, other):
        result = [a * b for a, b in zip(self.data, other.data)]
        return Array(result)

    def __truediv__(self, other):
        result = [a / b for a, b in zip(self.data, other.data)]
        return Array(result)

    def __pow__(self, exponent):
        result = [a ** exponent for a in self.data]
        return Array(result)

    def __neg__(self):
        result = [-a for a in self.data]
        return Array(result)

    def sum(self):
        return sum(self.data)

    def mean(self):
        return sum(self.data) / len(self.data)

    def product(self):
        result = 1
        for a in self.data:
            result *= a
        return result

    def min(self):
        return min(self.data)

    def max(self):
        return max(self.data)

    def sqrt(self):
        result = [a ** 0.5 for a in self.data]
        return Array(result)

    def sin(self):
        import math
        result = [math.sin(a) for a in self.data]
        return Array(result)

    def exp(self):
        import math
        result = [math.exp(a) for a in self.data]
        return Array(result)

    def log(self):
        import math
        result = [math.log(a) for a in self.data]
        return Array(result)

    def dot(self, other):
        result = sum([a * b for a, b in zip(self.data, other.data)])
        return Array([result])

def main():
    array1 = Array([1, 2, 3])
    array2 = Array([4, 5, 6])

    addition_result = array1 + array2
    subtraction_result = array1 - array2
    multiplication_result = array1 * array2
    division_result = array1 / array2
    exponentiation_result = array1 ** 2
    negation_result = -array1
    sum_result = array1.sum()
    mean_result = array1.mean()
    product_result = array1.product()
    min_result = array1.min()
    max_result = array1.max()
    sqrt_result = array1.sqrt()
    sin_result = array1.sin()
    exp_result = array1.exp()
    log_result = array1.log()
    dot_product_result = array1.dot(array2)

    print("Addition:", addition_result.data)
    print("Subtraction:", subtraction_result.data)
    print("Multiplication:", multiplication_result.data)
    print("Division:", division_result.data)
    print("Exponentiation:", exponentiation_result.data)
    print("Negation:", negation_result.data)
    print("Sum:", sum_result)
    print("Mean:", mean_result)
    print("Product:", product_result)
    print("Minimum:", min_result)
    print("Maximum:", max_result)
    print("Square Root:", sqrt_result.data)
    print("Sine:", sin_result.data)
    print("Exponential:", exp_result.data)
    print("Logarithm:", log_result.data)
    print("Dot Product:", dot_product_result.data)

if __name__ == "__main__":
    main()
