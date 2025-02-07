import numpy as np

class Array:
    def __init__(self, shape, data):
        self.shape = shape
        self.data = data

    def get_index(self, indices):
        return np.ravel_multi_index(indices, self.shape)

    def get(self, indices):
        index = self.get_index(indices)
        return self.data[index]

    def __add__(self, other):
        new_data = self.data + other.data
        return Array(self.shape, new_data)

    def __mul__(self, other):
        new_data = self.data * other.data
        return Array(self.shape, new_data)

    def dot(self, other):
        result_data = np.dot(self.data.reshape(-1, 1), other.data.reshape(1, -1))
        return Array((1,), result_data.flatten())

def main():
    vector_shape = (9,)
    matrix_shape = (3, 3)
    
    vector_data1 = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
    matrix_data1 = np.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]])

    vector_array1 = Array(vector_shape, vector_data1)
    matrix_array1 = Array(matrix_shape, matrix_data1)

    # Convert vector to a matrix with shape (1, 9)
    vector_as_matrix = Array((1, 9), vector_data1)

    # Add vector to matrix (broadcasting)
    result_add_vector_matrix = matrix_array1 + vector_as_matrix

    # Multiply vector by matrix (broadcasting)
    result_mul_vector_matrix = matrix_array1 * vector_as_matrix

    # Dot product of vector and matrix
    result_dot = vector_array1.dot(matrix_array1)

    print("Vector Array1:", vector_array1.data)
    print("Matrix Array1:\n", matrix_array1.data)

    print("Result of Adding Vector to Matrix:\n", result_add_vector_matrix.data)
    print("Result of Multiplying Vector by Matrix:\n", result_mul_vector_matrix.data)
    print("Dot Product Result:\n", result_dot.data)

if __name__ == "__main__":
    main()
