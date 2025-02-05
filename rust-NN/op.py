class Array:
    def __init__(self, shape, data):
        self.shape = shape
        self.data = data

    def get_index(self, indices):
        flat_index = 0
        stride = 1
        for i in range(len(indices) - 1, -1, -1):
            flat_index += indices[i] * stride
            stride *= self.shape[i]
        return flat_index

    def get(self, indices):
        index = self.get_index(indices)
        return self.data[index]

    def broadcast_shape(self, other_shape):
        result_shape = []
        for s1, s2 in zip(reversed(self.shape), reversed(other_shape)):
            if s1 == s2 or s1 == 1 or s2 == 1:
                result_shape.append(max(s1, s2))
            else:
                raise ValueError("Incompatible shapes for broadcasting")
        return tuple(reversed(result_shape))

    def broadcast_to(self, target_shape):
        if self.shape == target_shape:
            return self
        new_data = [self.get(self.indices_from_flat_index(i, self.shape))
                    for i in range(self.size(target_shape))]
        return Array(target_shape, new_data)

    def indices_from_flat_index(self, flat_index, shape):
        indices = []
        stride = 1
        for s in reversed(shape):
            indices.append((flat_index // stride) % s)
            stride *= s
        return list(reversed(indices))

    def size(self, shape=None):
        if shape is None:
            shape = self.shape
        size = 1
        for s in shape:
            size *= s
        return size

    def __add__(self, other):
        if self.shape != other.shape:
            new_shape = self.broadcast_shape(other.shape)
            array1 = self.broadcast_to(new_shape)
            array2 = other.broadcast_to(new_shape)
        else:
            array1 = self
            array2 = other
        new_data = [a + b for a, b in zip(array1.data, array2.data)]
        return Array(array1.shape, new_data)

    def __sub__(self, other):
        if self.shape != other.shape:
            new_shape = self.broadcast_shape(other.shape)
            array1 = self.broadcast_to(new_shape)
            array2 = other.broadcast_to(new_shape)
        else:
            array1 = self
            array2 = other
        new_data = [a - b for a, b in zip(array1.data, array2.data)]
        return Array(array1.shape, new_data)

    def __mul__(self, other):
        if self.shape != other.shape:
            new_shape = self.broadcast_shape(other.shape)
            array1 = self.broadcast_to(new_shape)
            array2 = other.broadcast_to(new_shape)
        else:
            array1 = self
            array2 = other
        new_data = [a * b for a, b in zip(array1.data, array2.data)]
        return Array(array1.shape, new_data)

    def __truediv__(self, other):
        if self.shape != other.shape:
            new_shape = self.broadcast_shape(other.shape)
            array1 = self.broadcast_to(new_shape)
            array2 = other.broadcast_to(new_shape)
        else:
            array1 = self
            array2 = other
        new_data = [a / b for a, b in zip(array1.data, array2.data)]
        return Array(array1.shape, new_data)

    def __pow__(self, exponent):
        new_data = [a ** exponent for a in self.data]
        return Array(self.shape, new_data)

    def __neg__(self):
        new_data = [-a for a in self.data]
        return Array(self.shape, new_data)

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
        new_data = [a ** 0.5 for a in self.data]
        return Array(self.shape, new_data)

    def sin(self):
        import math
        new_data = [math.sin(a) for a in self.data]
        return Array(self.shape, new_data)

    def exp(self):
        import math
        new_data = [math.exp(a) for a in self.data]
        return Array(self.shape, new_data)

    def log(self):
        import math
        new_data = [math.log(a) for a in self.data]
        return Array(self.shape, new_data)

    def dot(self, other):
        result = sum([a * b for a, b in zip(self.data, other.data)])
        return Array((1,), [result])

def main():
    array_shape1 = (2, 3)
    array_shape2 = (1, 3)

    array_data1 = [1, 2, 3, 4, 5, 6]
    array_data2 = [7, 8, 9]

    array1 = Array(array_shape1, array_data1)
    array2 = Array(array_shape2, array_data2)

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
