use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
use web_sys::console;
use futures::channel::oneshot;
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::RefCell;

// Macro pour afficher des messages dans la console du navigateur
macro_rules! console_log {
    ($($t:tt)*) => (console::log_1(&format_args!($($t)*).to_string().into()))
}

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
    }).await.ok_or_else(|| JsValue::from_str("Failed to find an appropriate adapter"))?;

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Device"),
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
    }, None).await.map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    // Chargement du code WGSL
    let shader_source = include_str!("../shader.wgsl"); // Assurez-vous que le chemin est correct

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
        size: ((batch_size * output_size) as usize * std::mem::size_of::<f32>()) as u64,
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

    // Fonction helper pour mapper le buffer de manière asynchrone
    async fn map_buffer_async(device: &wgpu::Device, buffer_slice: wgpu::BufferSlice<'_>) -> Result<(), JsValue> {
        let (sender, receiver) = oneshot::channel();

        buffer_slice.map_async(wgpu::MapMode::Read, move |v| {
            sender.send(v).unwrap();
        });

        device.poll(wgpu::Maintain::Wait);

        receiver.await.unwrap().map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    // Récupération des poids mis à jour
    {
        let buffer_slice = weight_buffer.slice(..);
        map_buffer_async(&device, buffer_slice).await?;

        let data = buffer_slice.get_mapped_range();
        let updated_weights: &[f32] = bytemuck::cast_slice(&data);
        console_log!("Poids mis à jour : {:?}", &updated_weights[0..10]); // Afficher les 10 premiers poids pour l'exemple
        drop(data);
        weight_buffer.unmap();
    }

    // Récupération des sorties
    {
        let buffer_slice = output_buffer.slice(..);
        map_buffer_async(&device, buffer_slice).await?;

        let data = buffer_slice.get_mapped_range();
        let outputs: &[f32] = bytemuck::cast_slice(&data);
        console_log!("Sorties : {:?}", &outputs[0..10]); // Afficher les 10 premières sorties pour l'exemple
        drop(data);
        output_buffer.unmap();
    }

    Ok(())
}
