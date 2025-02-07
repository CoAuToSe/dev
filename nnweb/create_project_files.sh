#!/bin/bash

echo "Création du projet..."

# Nom du projet
PROJECT_NAME="neural_network_gpu"

# Créer un nouveau projet Rust
cargo new $PROJECT_NAME --lib
cd $PROJECT_NAME

# Mettre à jour le Cargo.toml
cat > Cargo.toml <<EOL
[package]
name = "$PROJECT_NAME"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
wgpu = { version = "0.15", features = ["webgl"] }
js-sys = "0.3"
console_error_panic_hook = "0.1"
EOL

# Créer le fichier src/lib.rs
cat > src/lib.rs <<EOL
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wgpu::util::DeviceExt;
use web_sys::console;

// Macro pour afficher des messages dans la console du navigateur
macro_rules! console_log {
    (\$($t:tt)*) => (console::log_1(&format_args!(\$($t)*).to_string().into()))
);

// Permet d'attraper les paniques et de les afficher dans la console du navigateur
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Fonction principale exportée vers JavaScript
#[wasm_bindgen]
pub async fn run_optimized_neural_network_training() -> Result<(), JsValue> {
    // Initialisation de l'adaptateur et du dispositif
    let instance = wgpu::Instance::default();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await.ok_or("Failed to find an appropriate adapter")?;

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Device"),
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
    }, None).await?;

    // Chargement du code WGSL
    let shader_source = include_str!("shader.wgsl");

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    // Dimensions du réseau
    let batch_size = 1024u32; // Nombre d'exemples dans le batch
    let input_size = 784u32; // Taille de l'entrée (par exemple, 28x28 pixels pour MNIST)
    let output_size = 10u32; // Nombre de classes de sortie

    // Données d'entrée aléatoires pour l'exemple
    let input_data_array = vec![0.5f32; (batch_size * input_size) as usize];
    let weight_data_array = vec![0.01f32; (input_size * output_size) as usize];
    let target_data_array = vec![1.0f32; (batch_size * output_size) as usize];
    let output_data_array = vec![0.0f32; (batch_size * output_size) as usize];

    // Création des buffers
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        contents: bytemuck::cast_slice(&input_data_array),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let weight_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Weight Buffer"),
        contents: bytemuck::cast_slice(&weight_data_array),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let target_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Target Buffer"),
        contents: bytemuck::cast_slice(&target_data_array),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: (output_data_array.len() * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Création du pipeline de calcul
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });

    // Création du bind group
    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: weight_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: target_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    // Enregistrement des commandes
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);

        // Calcul du nombre de groupes de travail
        let workgroup_count_x = (batch_size + 15) / 16;
        let workgroup_count_y = (output_size + 15) / 16;
        compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    // Soumission des commandes
    queue.submit(Some(encoder.finish()));

    // Attendre la fin des opérations
    device.poll(wgpu::Maintain::Wait);

    // Récupération des poids mis à jour
    {
        let buffer_slice = weight_buffer.slice(..);
        let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
        device.poll(wgpu::Maintain::Wait);
        mapping.await.map_err(|_| JsValue::from_str("Failed to map weight buffer"))?;
        let data = buffer_slice.get_mapped_range();
        let updated_weights: &[f32] = bytemuck::cast_slice(&data);
        console_log!("Poids mis à jour : {:?}", &updated_weights[0..10]); // Afficher les 10 premiers poids pour l'exemple
        drop(data);
        weight_buffer.unmap();
    }

    // Récupération des sorties
    {
        let buffer_slice = output_buffer.slice(..);
        let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
        device.poll(wgpu::Maintain::Wait);
        mapping.await.map_err(|_| JsValue::from_str("Failed to map output buffer"))?;
        let data = buffer_slice.get_mapped_range();
        let outputs: &[f32] = bytemuck::cast_slice(&data);
        console_log!("Sorties : {:?}", &outputs[0..10]); // Afficher les 10 premières sorties pour l'exemple
        drop(data);
        output_buffer.unmap();
    }

    Ok(())
}
EOL

# Créer le fichier shader.wgsl
cat > shader.wgsl <<EOL
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

    let batch_size = 1024u; // Doit correspondre à batch_size en Rust
    let input_size = 784u;  // Doit correspondre à input_size en Rust
    let output_size = 10u;  // Doit correspondre à output_size en Rust

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

        // Mise à jour des poids (descente de gradient)
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
EOL

# Créer le dossier www et le fichier index.html
mkdir www
cat > www/index.html <<EOL
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <title>Réseau de Neurones avec WebGPU en Rust</title>
</head>
<body>
    <script type="module">
        import init, { run_optimized_neural_network_training } from '../pkg/neural_network_gpu.js';

        async function run() {
            await init();
            await run_optimized_neural_network_training();
        }

        run();
    </script>
</body>
</html>
EOL

echo "Projet créé avec succès."
