struct WGPU_handle<T> {
    device: T,
    shader: T,
    surface: T,
    surface_config: T,
    uniforms: T,
    render_pipeline: T,
    swapchain: T,
    pipeline: T,
}

struct GUI<T> {
    window_handler: T,
    wgpu_handler: WGPU_handle<T>,
}

fn main() {
    println!("Hello, world!");
}
