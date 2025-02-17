#![allow(unused_variables)]
// #![windows_subsystem = "windows"] //remove the console when launched
use clap::Parser;
use futures::executor::block_on;
use notify::{RawEvent, RecommendedWatcher, Watcher};
use std::{
    self,
    borrow::Cow,
    fs::{read_to_string, OpenOptions},
    io::Write,
    ops::Add,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread::JoinHandle,
    time::{Duration, Instant},
    *,
};
use wgpu::{
    self,
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BufferBindingType, BufferUsages, CommandEncoderDescriptor, Device,
    DeviceDescriptor, Features, Instance, Limits, LoadOp, Operations, PipelineLayout,
    PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RequestAdapterOptions, ShaderModule, ShaderSource, ShaderStages, Surface, SurfaceConfiguration,
    TextureFormat, *,
};
use wgpu_subscriber;
use winit::{
    self,
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
    *,
};

#[derive(Debug)]
enum CustomEvent<'a> {
    Som(winit::event::Event<'a, DeviceEvent>),
    Reload,
    Non,
}

#[derive(Debug)]
struct Reload;

#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Opts {
    wgsl_file: PathBuf,

    #[clap(short, long)]
    create: bool,

    #[clap(short, long)]
    always_on_top: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
struct Uniforms {
    pub mouse: [f32; 2],
    pub time: f32,
}

impl Default for Uniforms {
    fn default() -> Uniforms {
        Uniforms {
            time: 0.,
            mouse: [0.0, 0.0],
        }
    }
}

impl Uniforms {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

struct Playground {
    watch_path: PathBuf,
    watch_path2: PathBuf,
    render_pipeline: RenderPipeline,
    window: Window,
    device: Device,
    vertex_shader_module: ShaderModule,
    pipeline_layout: PipelineLayout,
    swapchain_format: TextureFormat,
    surface_config: SurfaceConfiguration,
    surface: Surface,

    uniforms: Uniforms,
    flicker: u8,
}

impl Playground {
    fn reload(&mut self) {
        println!("Reload. {:?} {:?}", &self.watch_path, &self.flicker);
        self.flicker = match self.flicker {
            255 => 0,
            num => num + 1,
        };
        let mut temp_pathbuf: &PathBuf;
        if self.flicker % 2 == 0 {
            temp_pathbuf = &self.watch_path
        } else {
            temp_pathbuf = &self.watch_path2
        }

        // self.recreate_pipeline();

        match Self::create_pipeline(
            &self.device,
            &self.vertex_shader_module,
            &self.pipeline_layout,
            self.swapchain_format,
            temp_pathbuf,
        ) {
            Ok(render_pipeline) => self.render_pipeline = render_pipeline,
            Err(e) => println!("{}", e),
        }
        self.window.request_redraw();
    }

    fn listen(watch_path: PathBuf, proxy: EventLoopProxy<Reload>) {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();

        watcher
            .watch(&watch_path, notify::RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent {
                    path: Some(_),
                    op: Ok(_),
                    ..
                }) => {
                    proxy.send_event(Reload).unwrap();
                }
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    async fn get_async_stuff(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue) {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                // // /// Adapter that uses the least possible power. This is often an integrated GPU.
                // // LowPower = 0,
                // power_preference: wgpu::PowerPreference::LowPower,
                // /// Adapter that has the highest performance. This is often a discrete GPU.
                // HighPerformance = 1,
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(surface),
                // force_fallback_adapter: false, //default
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::default(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        (adapter, device, queue)
    }

    fn recreate_pipeline(&mut self) {
        match Self::create_pipeline(
            &self.device,
            &self.vertex_shader_module,
            &self.pipeline_layout,
            self.swapchain_format,
            &self.watch_path,
        ) {
            Ok(render_pipeline) => self.render_pipeline = render_pipeline,
            Err(e) => println!("{}", e),
        }
    }

    fn create_pipeline(
        device: &Device,
        vertex_shader_module: &ShaderModule,
        pipeline_layout: &PipelineLayout,
        swapchain_format: TextureFormat,
        frag_shader_path: &Path,
    ) -> core::result::Result<RenderPipeline, String> {
        // let frag_wgsl = read_to_string(&frag_shader_path).unwrap();
        let frag_wgsl = String::from(
            "struct VertexOutput {
    [[location(0)]] coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let r: f32 = dot(in.coord, in.coord);

    if (r > .95) {
        discard;
    }

    let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;
    return vec4<f32>(normalized.rg, 0., 1.0);
}",
        );

        let fragement_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Fragment shader"),
            source: ShaderSource::Wgsl(Cow::Owned(frag_wgsl)),
        });

        Ok(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader_module,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &fragement_shader_module,
                    entry_point: "fs_main",
                    targets: &[swapchain_format.into()],
                }),
            }),
        )
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        self.surface_config.width = new_size.width;
        self.surface_config.height = new_size.height;

        self.surface.configure(&self.device, &self.surface_config);
        self.window.request_redraw();
    }

    pub fn run(opts: &Opts) {
        let event_loop: EventLoop<Reload> = EventLoop::with_user_event();
        let proxy = event_loop.create_proxy();

        let proxy2 = event_loop.create_proxy();
        std::thread::spawn(move || loop {
            proxy2.send_event(Reload).unwrap();
            std::thread::sleep(std::time::Duration::new(5, 0));
        });

        {
            let watch_path = opts.wgsl_file.clone();
            std::thread::spawn(move || Self::listen(watch_path, proxy));
        }

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(600, 600))
            .with_title("WGSL Playground")
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();

        window.set_always_on_top(opts.always_on_top);

        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let (adapter, device, queue) = block_on(Self::get_async_stuff(&instance, &surface));

        let vertex_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./vertex.wgsl").into()),
        });

        let uniforms = Uniforms::default();

        let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: uniforms.as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniforms_buffer_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniforms_buffer_layout],
            push_constant_ranges: &[],
        });

        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

        let render_pipeline = match Self::create_pipeline(
            &device,
            &vertex_shader_module,
            &pipeline_layout,
            swapchain_format,
            &opts.wgsl_file,
        ) {
            Ok(render_pipeline) => render_pipeline,
            Err(e) => {
                println!("Could not start due to error: {}", &e);
                return;
            }
        };

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox, // Vsync or better (try 60fps or more if possible (overkill))
                                                      // present_mode: wgpu::PresentMode::Fifo, // Vsync (try to have 60 fps)
        };

        surface.configure(&device, &surface_config);

        let uniforms_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &uniforms_buffer_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
        });

        let mut playground = Playground {
            watch_path: opts.wgsl_file.clone(),
            watch_path2: " C:\\Users\\Aurélien\\Documents\\dev\\own_online_stuf\\wgsl-playground-main\\examples\\circle.wgsl".into(),
            render_pipeline,
            window,
            device,
            swapchain_format,
            pipeline_layout,
            vertex_shader_module,
            surface_config,
            surface,
            uniforms,
            flicker: 0u8,
        };

        let instant = Instant::now();
        let mut drawing = std::sync::Arc::new(std::sync::Mutex::new(false));
        let mut drawind_thread = std::thread::spawn(|| {});
        let mut drawind_thread2 = std::thread::spawn(|| {});
        let mut drawind_thread3 = std::thread::spawn(|| {});
        let mut index = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));
        let mut true_index = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));
        let mut timer = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));

        let data = std::sync::Arc::clone(&timer);
        std::thread::spawn(move || loop {
            {
                let mut timer_in = data.lock().unwrap();
                // println!("timer {}", *timer_in);
                *timer_in = match *timer_in {
                    usize::MAX => 0,
                    num => num + 1,
                };
            }
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000));
        });

        let data = std::sync::Arc::clone(&index);
        let data2 = std::sync::Arc::clone(&timer);
        let data3 = std::sync::Arc::clone(&true_index);
        std::thread::spawn(move || loop {
            {
                let index_in = data.lock().unwrap();
                let timer_in = data2.lock().unwrap();
                let true_index_in = data3.lock().unwrap();
                println!(
                    "{} | {}: {} | {}: {}",
                    *timer_in,
                    *index_in,
                    *index_in / *timer_in,
                    *true_index_in,
                    *true_index_in / *timer_in,
                );
            }
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000));
        });

        event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;
            *control_flow = ControlFlow::Poll;
            match event {
                winit::event::Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => playground.resize(new_size),
                    WindowEvent::CursorMoved { position, .. } => {
                        let size = playground.window.inner_size();
                        let normalized_x = position.x as f32 / size.width as f32;
                        let normalized_y = position.y as f32 / size.height as f32;
                        playground.uniforms.mouse =
                            [normalized_x * 2. - 1., -normalized_y * 2. + 1.];
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        playground.resize(new_inner_size)
                    }
                    _ => {}
                },
                winit::event::Event::RedrawRequested(_) => {
                    let data = std::sync::Arc::clone(&drawing);
                    let internal_drawing;
                    // println!("trying to draw!");
                    {
                        let mut is_drawing = data.lock().unwrap();
                        if !*is_drawing {
                            // std::thread::sleep(std::time::Duration::new(0, 1_000_000));
                            *is_drawing = true;
                            internal_drawing = true;
                            // println!("will draw!");
                            let datai = std::sync::Arc::clone(&true_index);
                            drawind_thread2 = std::thread::spawn(move || {
                                let mut index_in = datai.lock().unwrap();
                                *index_in = match *index_in {
                                    usize::MAX => 0,
                                    num => num + 1,
                                };
                            });
                        } else {
                            internal_drawing = false;
                            // println!("nice try!");
                        }
                    }
                    if !internal_drawing {
                        playground.uniforms.time = instant.elapsed().as_secs_f32();
                        queue.write_buffer(&uniforms_buffer, 0, playground.uniforms.as_bytes());
                        let output_frame = playground.surface.get_current_frame().unwrap();
                        let view = output_frame
                            .output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = playground
                            .device
                            .create_command_encoder(&CommandEncoderDescriptor { label: None });

                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: Operations {
                                            load: LoadOp::Clear(wgpu::Color::BLACK),
                                            store: true,
                                        },
                                    }],
                                    depth_stencil_attachment: None,
                                });
                            render_pass.set_pipeline(&playground.render_pipeline);
                            render_pass.set_bind_group(0, &uniforms_buffer_bind_group, &[]);
                            render_pass.draw(0..3, 0..1);
                        }

                        queue.submit(Some(encoder.finish()));
                        let data2 = std::sync::Arc::clone(&drawing);
                        drawind_thread = std::thread::spawn(move || {
                            // std::thread::sleep(std::time::Duration::new(0, 500_000_000));
                            let mut is_drawing2 = data2.lock().unwrap();
                            *is_drawing2 = false;
                        });
                    }
                }
                winit::event::Event::UserEvent(Reload) => {
                    // println!("User Event");
                    playground.reload();
                }
                winit::event::Event::MainEventsCleared => {
                    let data = std::sync::Arc::clone(&index);
                    drawind_thread3 = std::thread::spawn(move || {
                        let mut index_in = data.lock().unwrap();
                        *index_in = match *index_in {
                            usize::MAX => 0,
                            num => num + 1,
                        };
                    });
                    // println!("Main Events Cleared");
                    playground.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
fn create_pipeline(
    device: &Device,
    vertex_shader_module: &ShaderModule,
    pipeline_layout: &PipelineLayout,
    swapchain_format: TextureFormat,
    frag_shader_path: &Path,
) -> core::result::Result<RenderPipeline, String> {
    let frag_wgsl = read_to_string(&frag_shader_path).unwrap();

    let fragement_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Fragment shader"),
        source: ShaderSource::Wgsl(Cow::Owned(frag_wgsl)),
    });

    Ok(
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader_module,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragement_shader_module,
                entry_point: "fs_main",
                targets: &[swapchain_format.into()],
            }),
        }),
    )
}
async fn get_async_stuff(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue) {
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            // // /// Adapter that uses the least possible power. This is often an integrated GPU.
            // // LowPower = 0,
            // power_preference: wgpu::PowerPreference::LowPower,
            // /// Adapter that has the highest performance. This is often a discrete GPU.
            // HighPerformance = 1,
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
            // force_fallback_adapter: false, //default
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::default(),
                limits: Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    (adapter, device, queue)
}

fn listen(watch_path: PathBuf, proxy: EventLoopProxy<CustomEvent>) {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();

    watcher
        .watch(&watch_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(_),
                op: Ok(_),
                ..
            }) => {
                proxy.send_event(CustomEvent::Reload).unwrap();
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn main() {
    println!("Hello, world!");
    wgpu_subscriber::initialize_default_subscriber(None);
    let opts = Opts::parse();

    if opts.create {
        let mut file = if let Ok(file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(opts.wgsl_file.clone())
        {
            file
        } else {
            println!(
                "Couldn't create file {:?}, make sure it doesn't already exist.",
                &opts.wgsl_file
            );
            return;
        };
        file.write_all(include_bytes!("frag.default.wgsl")).unwrap();
    }
    let event_loop = event_loop::EventLoop::<CustomEvent>::with_user_event();
    let my_window = window::WindowBuilder::new()
        .with_always_on_top(false)
        .with_decorations(true)
        .with_fullscreen(None)
        .with_inner_size(dpi::LogicalSize::new(200, 200))
        .with_max_inner_size(dpi::LogicalSize::new(1000, 1000))
        .with_maximized(false)
        .with_position(dpi::LogicalPosition::new(100, 100))
        .with_resizable(true)
        .with_title("My Title")
        .with_transparent(false)
        .with_visible(true)
        .with_window_icon(None)
        .build(&event_loop)
        .unwrap();
    //
    let event_loop_proxy = event_loop.create_proxy();
    let my_window_id = my_window.id();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::new(5, 0));
        event_loop_proxy
            .send_event(CustomEvent::Som(winit::event::Event::DeviceEvent {
                device_id: unsafe { winit::event::DeviceId::dummy() },
                event: winit::event::DeviceEvent::MouseMotion { delta: (0.0, 10.0) },
            }))
            .ok();

        event_loop_proxy
            .send_event(CustomEvent::Som(winit::event::Event::WindowEvent {
                window_id: my_window_id,
                event: winit::event::WindowEvent::Focused(true),
            }))
            .ok();
    });

    // let event_loop: EventLoop<Reload> = EventLoop::with_user_event();
    let proxy = event_loop.create_proxy();

    let proxy2 = event_loop.create_proxy();
    std::thread::spawn(move || loop {
        proxy2.send_event(CustomEvent::Reload).unwrap();
        std::thread::sleep(std::time::Duration::new(5, 0));
    });

    {
        let watch_path = opts.wgsl_file.clone();
        std::thread::spawn(move || listen(watch_path, proxy));
    }

    // let window = WindowBuilder::new()
    //     .with_inner_size(PhysicalSize::new(600, 600))
    //     .with_title("WGSL Playground")
    //     .build(&event_loop)
    //     .unwrap();
    let size = &my_window.inner_size();

    my_window.set_always_on_top(opts.always_on_top);

    let instance = wgpu::Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(&my_window) };
    let (adapter, device, queue) = block_on(get_async_stuff(&instance, &surface));

    let vertex_shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Vertex shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./vertex.wgsl").into()),
    });

    let uniforms = Uniforms::default();

    let uniforms_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: uniforms.as_bytes(),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let uniforms_buffer_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            count: None,
            ty: wgpu::BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&uniforms_buffer_layout],
        push_constant_ranges: &[],
    });

    let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

    let render_pipeline = match create_pipeline(
        &device,
        &vertex_shader_module,
        &pipeline_layout,
        swapchain_format,
        &opts.wgsl_file,
    ) {
        Ok(render_pipeline) => render_pipeline,
        Err(e) => {
            println!("Could not start due to error: {}", &e);
            return;
        }
    };

    let surface_config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        // present_mode: wgpu::PresentMode::Mailbox, // Vsync or better (try 60fps or more if possible (overkill))
        //
        present_mode: wgpu::PresentMode::Fifo, // Vsync (try to have 60 fps)
    };

    surface.configure(&device, &surface_config);

    let uniforms_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &uniforms_buffer_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: uniforms_buffer.as_entire_binding(),
        }],
    });

    let mut playground = Playground {
            watch_path: opts.wgsl_file.clone(),
            watch_path2: "C:\\Users\\Aurel\\OneDrive\\Documents\\GitHub\\own_online_stuf\\wgsl-playground-main\\examples\\circle.wgsl".into(),
            render_pipeline,
            window: my_window,
            device,
            swapchain_format,
            pipeline_layout,
            vertex_shader_module,
            surface_config,
            surface,
            uniforms,
            flicker: 0u8,
        };

    let instant = Instant::now();
    let mut drawing = std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut drawind_thread = std::thread::spawn(|| {});
    let mut drawind_thread2 = std::thread::spawn(|| {});
    let mut drawind_thread3 = std::thread::spawn(|| {});
    let mut index = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));
    let mut true_index = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));
    let mut timer = std::sync::Arc::new(std::sync::Mutex::new(0 as usize));

    let data = std::sync::Arc::clone(&timer);
    std::thread::spawn(move || loop {
        {
            let mut timer_in = data.lock().unwrap();
            // println!("timer {}", *timer_in);
            *timer_in = match *timer_in {
                usize::MAX => 0,
                num => num + 1,
            };
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000));
    });

    let data = std::sync::Arc::clone(&index);
    let data2 = std::sync::Arc::clone(&timer);
    let data3 = std::sync::Arc::clone(&true_index);
    std::thread::spawn(move || loop {
        {
            let index_in = data.lock().unwrap();
            let timer_in = data2.lock().unwrap();
            let true_index_in = data3.lock().unwrap();
            println!(
                "{} | {}: {} | {}: {}",
                *timer_in,
                *index_in,
                *index_in / *timer_in,
                *true_index_in,
                *true_index_in / *timer_in,
            );
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000));
    });
    event_loop.run(move |event, something, control_flow| {
        // println!("{:?} {:?} {:?}", event, something, control_flow);
        *control_flow = ControlFlow::Poll;
        // *control_flow = ControlFlow::Wait;
        // *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10000));
        // *control_flow = ControlFlow::Exit;
        match event {
            Event::NewEvents(some) => match some {
                event::StartCause::ResumeTimeReached {
                    start,
                    requested_resume,
                } => (),
                event::StartCause::WaitCancelled {
                    start,
                    requested_resume,
                } => (),
                event::StartCause::Poll => (),
                event::StartCause::Init => (),
            },
            Event::WindowEvent { window_id, event } => match event {
                event::WindowEvent::Resized(new_size) => playground.resize(&new_size),
                event::WindowEvent::Moved(some) => (),
                event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                event::WindowEvent::Destroyed => *control_flow = ControlFlow::Exit,
                event::WindowEvent::DroppedFile(some) => (),
                event::WindowEvent::HoveredFile(some) => (),
                event::WindowEvent::HoveredFileCancelled => (),
                event::WindowEvent::ReceivedCharacter(some) => (),
                event::WindowEvent::Focused(some) => (),
                event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => (),
                event::WindowEvent::ModifiersChanged(some) => (),
                event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                    modifiers,
                } => {
                    let size = playground.window.inner_size();
                    let normalized_x = position.x as f32 / size.width as f32;
                    let normalized_y = position.y as f32 / size.height as f32;
                    playground.uniforms.mouse = [normalized_x * 2. - 1., -normalized_y * 2. + 1.];
                }
                event::WindowEvent::CursorEntered { device_id } => (),
                event::WindowEvent::CursorLeft { device_id } => (),
                event::WindowEvent::MouseWheel {
                    device_id,
                    delta,
                    phase,
                    modifiers,
                } => (),
                event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers,
                } => (),
                event::WindowEvent::TouchpadPressure {
                    device_id,
                    pressure,
                    stage,
                } => (),
                event::WindowEvent::AxisMotion {
                    device_id,
                    axis,
                    value,
                } => (),
                event::WindowEvent::Touch(some) => (),
                event::WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => playground.resize(new_inner_size),
                event::WindowEvent::ThemeChanged(some) => (),
            },
            Event::DeviceEvent { device_id, event } => match event {
                DeviceEvent::Added => (),
                DeviceEvent::Removed => (),
                DeviceEvent::MouseMotion { delta } => (),
                DeviceEvent::MouseWheel { delta } => (),
                DeviceEvent::Motion { axis, value } => (),
                DeviceEvent::Button { button, state } => (),
                DeviceEvent::Key(some) => (),
                DeviceEvent::Text { codepoint } => (),
            },
            Event::UserEvent(some) => match some {
                CustomEvent::Reload => {
                    // println!("User Event");
                    playground.reload();
                }
                _ => (),
            },
            Event::Suspended => (),
            Event::Resumed => (),
            Event::MainEventsCleared => {
                let data = std::sync::Arc::clone(&index);
                drawind_thread3 = std::thread::spawn(move || {
                    let mut index_in = data.lock().unwrap();
                    *index_in = match *index_in {
                        usize::MAX => 0,
                        num => num + 1,
                    };
                });
                // println!("Main Events Cleared");
                playground.window.request_redraw();
            }
            Event::RedrawRequested(some) => {
                let data = std::sync::Arc::clone(&drawing);
                let internal_drawing;
                // println!("trying to draw!");
                {
                    let mut is_drawing = data.lock().unwrap();
                    if !*is_drawing {
                        // std::thread::sleep(std::time::Duration::new(0, 1_000_000));
                        *is_drawing = true;
                        internal_drawing = true;
                        // println!("will draw!");
                        let datai = std::sync::Arc::clone(&true_index);
                        drawind_thread2 = std::thread::spawn(move || {
                            let mut index_in = datai.lock().unwrap();
                            *index_in = match *index_in {
                                usize::MAX => 0,
                                num => num + 1,
                            };
                        });
                    } else {
                        internal_drawing = false;
                        // println!("nice try!");
                    }
                }
                if !internal_drawing {
                    playground.uniforms.time = instant.elapsed().as_secs_f32();
                    queue.write_buffer(&uniforms_buffer, 0, playground.uniforms.as_bytes());
                    let output_frame = playground.surface.get_current_frame().unwrap();
                    let view = output_frame
                        .output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = playground
                        .device
                        .create_command_encoder(&CommandEncoderDescriptor { label: None });

                    {
                        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                            label: None,
                            color_attachments: &[RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: Operations {
                                    load: LoadOp::Clear(wgpu::Color::BLACK),
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: None,
                        });
                        render_pass.set_pipeline(&playground.render_pipeline);
                        render_pass.set_bind_group(0, &uniforms_buffer_bind_group, &[]);
                        render_pass.draw(0..3, 0..1);
                    }

                    queue.submit(Some(encoder.finish()));
                    let data2 = std::sync::Arc::clone(&drawing);
                    drawind_thread = std::thread::spawn(move || {
                        // std::thread::sleep(std::time::Duration::new(0, 500_000_000));
                        let mut is_drawing2 = data2.lock().unwrap();
                        *is_drawing2 = false;
                    });
                }
            }
            Event::RedrawEventsCleared => (),
            Event::LoopDestroyed => (),
        }
    });
}
