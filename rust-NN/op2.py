import numpy as np

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

    def __mul__(self, other):
        if isinstance(other, Array):
            if self.shape != other.shape:
                new_shape = self.broadcast_shape(other.shape)
                array1 = self.broadcast_to(new_shape)
                array2 = other.broadcast_to(new_shape)
            else:
                array1 = self
                array2 = other
            new_data = [a * b for a, b in zip(array1.data, array2.data)]
            return Array(array1.shape, new_data)
        elif isinstance(other, int) or isinstance(other, float):
            new_data = [a * other for a in self.data]
            return Array(self.shape, new_data)

    # def dot(self, other):
    #     if isinstance(other, Array):
    #         if len(self.shape) == 2 and len(other.shape) == 2 and self.shape[1] == other.shape[0]:
    #             result_data = []
    #             for i in range(self.shape[0]):
    #                 row = []
    #                 for j in range(other.shape[1]):
    #                     element = 0
    #                     for k in range(self.shape[1]):
    #                         element += self.get((i, k)) * other.get((k, j))
    #                     row.append(element)
    #                 result_data.extend(row)
    #             return Array((self.shape[0], other.shape[1]), result_data)
    #         else:
    #             raise ValueError("Incompatible shapes for dot product")
    #     else:
    #         raise TypeError("Unsupported type for dot product")

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

    # def dot(self, other):
    #     result = sum([a * b for a, b in zip(self.data, other.data)])
    #     return Array((1,), [result])
    def dot(self, other):
        if isinstance(other, Array):
            if len(self.shape) == 2 and len(other.shape) == 2:
                if self.shape[1] == other.shape[0]:
                    result_data = []
                    for i in range(self.shape[0]):
                        row = []
                        for j in range(other.shape[1]):
                            element = sum(a * b for a, b in zip(self.get_row(i), other.get_column(j)))
                            row.append(element)
                        result_data.extend(row)
                    return Array((self.shape[0], other.shape[1]), result_data)
                else:
                    raise ValueError("Incompatible shapes for dot product")
            elif len(self.shape) == 1 and len(other.shape) == 1:
                result_data = sum(a * b for a, b in zip(self.data, other.data))
                return result_data
            elif len(self.shape) == 1 and len(other.shape) == 2 and self.shape[0] == other.shape[0]:
                result_data = [sum(a * b for a, b in zip(self.data, other.get_column(j))) for j in range(other.shape[1])]
                return Array((other.shape[1],), result_data)
            elif len(self.shape) == 2 and len(other.shape) == 1 and self.shape[1] == other.shape[0]:
                result_data = [sum(a * b for a, b in zip(self.get_row(i), other.data)) for i in range(self.shape[0])]
                return Array((self.shape[0],), result_data)
            elif self.shape == other.shape:  # Handle element-wise multiplication
                result_data = [a * b for a, b in zip(self.data, other.data)]
                return Array(self.shape, result_data)
            else:
                raise ValueError("Incompatible shapes for dot product")
        else:
            raise TypeError("Unsupported type for dot product")

    def get_row(self, index):
        return [self.get((index, j)) for j in range(self.shape[1])]

    def get_column(self, index):
        return [self.get((i, index)) for i in range(self.shape[0])]

def main():
    array_shape1 = (2, 3)
    array_shape2 = (1, 3)

    array_data1 = [1, 2, 3, 4, 5, 6]
    array_data2 = [7, 8, 9]

    array1 = Array(array_shape1, array_data1)
    array2 = Array(array_shape2, array_data2)

    # Your Array operations
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
    # dot_product_result = array1.dot(array2)

    # Convert Array instances to NumPy arrays
    np_array1 = np.array(array_data1).reshape(array_shape1)
    np_array2 = np.array(array_data2).reshape(array_shape2)

    # NumPy operations
    np_addition_result = np_array1 + np_array2
    np_subtraction_result = np_array1 - np_array2
    np_multiplication_result = np_array1 * np_array2
    np_division_result = np_array1 / np_array2
    np_exponentiation_result = np_array1 ** 2
    np_negation_result = -np_array1
    np_sum_result = np_array1.sum()
    np_mean_result = np_array1.mean()
    np_product_result = np_array1.prod()
    np_min_result = np_array1.min()
    np_max_result = np_array1.max()
    np_sqrt_result = np.sqrt(np_array1)
    np_sin_result = np.sin(np_array1)
    np_exp_result = np.exp(np_array1)
    np_log_result = np.log(np_array1)
    # np_dot_product_result = np.dot(np_array1, np_array2)

    # Compare results
    print("Addition (Array):", addition_result.data)
    print("Addition (NumPy):", np_addition_result)
    print("Arrays are equal:", np.array_equal(addition_result.data, np_addition_result))

    print("Subtraction (Array):", subtraction_result.data)
    print("Subtraction (NumPy):", np_subtraction_result)
    print("Arrays are equal:", np.array_equal(subtraction_result.data, np_subtraction_result))

    print("Multiplication (Array):", multiplication_result.data)
    print("Multiplication (NumPy):", np_multiplication_result)
    print("Arrays are equal:", np.array_equal(multiplication_result.data, np_multiplication_result))

    print("Division (Array):", division_result.data)
    print("Division (NumPy):", np_division_result)
    print("Arrays are equal:", np.array_equal(division_result.data, np_division_result))

    print("Exponentiation (Array):", exponentiation_result.data)
    print("Exponentiation (NumPy):", np_exponentiation_result)
    print("Arrays are equal:", np.array_equal(exponentiation_result.data, np_exponentiation_result))

    print("Negation (Array):", negation_result.data)
    print("Negation (NumPy):", np_negation_result)
    print("Arrays are equal:", np.array_equal(negation_result.data, np_negation_result))

    print("Sum (Array):", sum_result)
    print("Sum (NumPy):", np_sum_result)
    print("Values are equal:", sum_result == np_sum_result)
    
    print("Mean (Array):", mean_result)
    print("Mean (NumPy):", np_mean_result)
    print("Values are equal:", np.isclose(mean_result, np_mean_result))

    print("Product (Array):", product_result)
    print("Product (NumPy):", np_product_result)
    print("Values are equal:", np.isclose(product_result, np_product_result))

    print("Min (Array):", min_result)
    print("Min (NumPy):", np_min_result)
    print("Values are equal:", np.isclose(min_result, np_min_result))

    print("Max (Array):", max_result)
    print("Max (NumPy):", np_max_result)
    print("Values are equal:", np.isclose(max_result, np_max_result))

    print("Sqrt (Array):", sqrt_result.data)
    print("Sqrt (NumPy):", np_sqrt_result)
    print("Arrays are equal:", np.array_equal(sqrt_result.data, np_sqrt_result))

    print("Sin (Array):", sin_result.data)
    print("Sin (NumPy):", np_sin_result)
    print("Arrays are equal:", np.array_equal(sin_result.data, np_sin_result))

    print("Exp (Array):", exp_result.data)
    print("Exp (NumPy):", np_exp_result)
    print("Arrays are equal:", np.array_equal(exp_result.data, np_exp_result))

    print("Log (Array):", log_result.data)
    print("Log (NumPy):", np_log_result)
    print("Arrays are equal:", np.array_equal(log_result.data, np_log_result))
    
    array_shape1 = (2, 3)
    array_shape2 = (3, 2)
    array_shape3 = (2,)
    array_shape4 = (3,)
    array_shape5 = (3, 3)

    array_data1 = [1, 2, 3, 4, 5, 6]
    array_data2 = [7, 8, 9, 10, 11, 12]
    array_data3 = [2, 3]
    array_data4 = [4, 5, 6]
    array_data5 = [1, 2, 3, 4, 5, 6, 7, 8, 9]

    array1 = Array(array_shape1, array_data1)
    array2 = Array(array_shape2, array_data2)
    array3 = Array(array_shape3, array_data3)
    array4 = Array(array_shape4, array_data4)
    array5 = Array(array_shape5, array_data5)

    # Your Array dot operations
    dot_product1 = array1.dot(array2)
    # dot_product2 = array1.dot(array3)
    dot_product3 = array4.dot(array2)
    dot_product4 = array3.dot(array4)
    dot_product5 = array5.dot(array5)

    print("Dot Product 1 (Array):")
    print(dot_product1.data)
    print("Dot Product 2 (Array):")
    # print(dot_product2.data)
    print("Dot Product 3 (Array):")
    print(dot_product3.data)
    print("Dot Product 4 (Array):")
    # print(dot_product4.data)
    print("Dot Product 5 (Array):")
    print(dot_product5.data)

    # Convert Array instances to NumPy arrays
    np_array1 = np.array(array_data1).reshape(array_shape1)
    np_array2 = np.array(array_data2).reshape(array_shape2)
    np_array3 = np.array(array_data3).reshape(array_shape3)
    np_array4 = np.array(array_data4).reshape(array_shape4)
    np_array5 = np.array(array_data5).reshape(array_shape5)

    # NumPy dot operations
    np_dot_product1 = np.dot(np_array1, np_array2)
    # np_dot_product2 = np.dot(np_array1, np_array3)
    np_dot_product3 = np.dot(np_array4, np_array2)
    # np_dot_product4 = np.dot(np_array3, np_array4)
    np_dot_product5 = np.dot(np_array5, np_array5)

    print("Dot Product 1 (NumPy):")
    print(np_dot_product1)
    print("Dot Product 2 (NumPy):")
    # print(np_dot_product2)
    print("Dot Product 3 (NumPy):")
    print(np_dot_product3)
    print("Dot Product 4 (NumPy):")
    # print(np_dot_product4)
    print("Dot Product 5 (NumPy):")
    print(np_dot_product5)

    # Compare results
    print("Arrays are equal (Dot Product 1):", np.array_equal(dot_product1.data, np_dot_product1))
    # print("Arrays are equal (Dot Product 2):", np.array_equal(dot_product2.data, np_dot_product2))
    print("Arrays are equal (Dot Product 3):", np.array_equal(dot_product3.data, np_dot_product3))
    # print("Arrays are equal (Dot Product 4):", np.array_equal(dot_product4.data, np_dot_product4))
    print("Arrays are equal (Dot Product 5):", np.array_equal(dot_product5.data, np_dot_product5))



if __name__ == "__main__":
    main()
