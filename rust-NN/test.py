import numpy as np

def sigmoid(x):
    return 1 / (1 + np.exp(-x))

def sigmoid_derivative(x):
    return x * (1 - x)

def mse_loss(y_true, y_pred):
    return np.mean((y_true - y_pred)**2)

num_neuron_1 = 5

# Initialiser les poids et les biais aléatoirement
weights_1 = np.random.rand(3, num_neuron_1) # 3 entrées, 4 neurones dans la couche cachée # [ [f64 ; len_couche_cache] ; len_entre] / [[f64;5];3]
bias_1 = np.random.rand(num_neuron_1) # [f64 ; len_couche_cache]
weights_2 = np.random.rand(num_neuron_1, 1) # 4 neurones dans la couche cachée, 1 sortie # [ [f64 ; len_sortie] ; len_couche_cache] / [[f64;1];5]
bias_2 = np.random.rand(1) # [f64 ; len_sortie]

def train(x, y_true, epochs, learning_rate):
    """_summary_

    Args:
        x ([[f64;3];4]): inputs
        y_true ([f64;4]): outputs
        epochs (usize): num of iteration
        learning_rate (usize): size of steps
    """
    global weights_1, bias_1, weights_2, bias_2
    for epoch in range(epochs):
        # Forward propagation
        layer_1_output = sigmoid(np.dot(x, weights_1) + bias_1) # [[5]4] = f( [[3]4] * [[5]3] + [5] )
        # [[5]4] = [[3]4] * [[5]3]
        # [[5]4] = [[5]4] + [5] = [[5]4] + [5]x4 
        output = sigmoid(np.dot(layer_1_output, weights_2) + bias_2) # 

        # Backward propagation
        output_error = mse_loss(y_true, output) #
        d_output_error = 2 * (output - y_true) #

        d_weights_2 = np.dot(layer_1_output.T, d_output_error * sigmoid_derivative(output)) # 
        d_bias_2 = np.sum(d_output_error * sigmoid_derivative(output), axis=0) # 

        d_layer_1_error = np.dot(d_output_error * sigmoid_derivative(output), weights_2.T)
        d_weights_1 = np.dot(x.T, d_layer_1_error * sigmoid_derivative(layer_1_output))
        d_bias_1 = np.sum(d_layer_1_error * sigmoid_derivative(layer_1_output), axis=0)

        # Update weights and biases
        weights_1 -= learning_rate * d_weights_1
        bias_1 -= learning_rate * d_bias_1
        weights_2 -= learning_rate * d_weights_2
        bias_2 -= learning_rate * d_bias_2

        if epoch % 100 == 0:
            print(f"Epoch {epoch}, Loss: {output_error} ;\nweights_1\n{weights_1};\nx\n{x};\nbias_1\n{bias_1};\nnp.dot(x, weights_1)\n{np.dot(x, weights_1)};\nnp.dot(x, weights_1) + bias_1\n{np.dot(x, weights_1) + bias_1};\nlayer_1_output\n{layer_1_output};\noutput\n{output};\nlayer_1_output.T\n{layer_1_output.T};\nd_output_error\n{d_output_error};\nsigmoid_derivative(output)\n{sigmoid_derivative(output)};\nd_output_error * sigmoid_derivative(output)\n{d_output_error * sigmoid_derivative(output)};\nnp.dot(layer_1_output.T, d_output_error * sigmoid_derivative(output))\n{np.dot(layer_1_output.T, d_output_error * sigmoid_derivative(output))};")

# Pour tester
inputs = np.array([[0, 0, 1], [0, 1, 1], [1, 0, 1], [1, 1, 1]])
outputs = np.array([[0], [1], [1], [0]])

train(inputs, outputs, epochs=10000, learning_rate=0.1)