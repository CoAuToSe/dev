struct Matrix {
    data: array<f32>,
};

@group(0) @binding(0)
var<storage, read> inputData: Matrix; // [batch_size * input_size]

@group(0) @binding(1)
var<storage, read_write> weights: Matrix; // [input_size * output_size]

@group(0) @binding(2)
var<storage, read> targetData: Matrix; // [batch_size * output_size]

@group(0) @binding(3)
var<storage, write> outputData: Matrix; // [batch_size * output_size]

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let batch_idx = global_id.x;
    let output_idx = global_id.y;

    let batch_size = 1024u; // Doit correspondre Ã  batch_size en Rust
    let input_size = 784u;  // Doit correspondre Ã  input_size en Rust
    let output_size = 10u;  // Doit correspondre Ã  output_size en Rust

    if (batch_idx < batch_size && output_idx < output_size) {
        var output: f32 = 0.0;
        for (var i: u32 = 0u; i < input_size; i = i + 1u) {
            let input_idx = batch_idx * input_size + i;
            let weight_idx = i * output_size + output_idx;
            output = output + inputData.data[input_idx] * weights.data[weight_idx];
        }

        // Fonction d'activation ReLU
        output = max(0.0, output);

        // Calcul de l'erreur
        let target_idx = batch_idx * output_size + output_idx;
        let error = output - targetData.data[target_idx];

        // Taux d'apprentissage
        let learningRate = 0.01;

        // Mise Ã  jour des poids (descente de gradient)
        for (var i: u32 = 0u; i < input_size; i = i + 1u) {
            let input_idx = batch_idx * input_size + i;
            let weight_idx = i * output_size + output_idx;
            let gradient = error * inputData.data[input_idx];
            weights.data[weight_idx] = weights.data[weight_idx] - learningRate * gradient;
        }

        // Stockage de la sortie
        outputData.data[target_idx] = output;
    }
}
