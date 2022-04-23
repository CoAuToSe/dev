use std::time::Duration;

use winit::{self, *};

fn main() {
    println!("Hello, world!");
    let mut event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new()
        .with_always_on_top(false)
        .with_decorations(true)
        .with_fullscreen(None)
        .with_inner_size(dpi::LogicalSize::new(100, 100))
        .with_max_inner_size(dpi::LogicalSize::new(1000, 1000))
        .with_maximized(false)
        .with_position(dpi::LogicalPosition::new(10, 10))
        .with_resizable(true)
        .with_title("My Title")
        .with_transparent(false)
        .with_visible(true)
        .with_window_icon(None)
        .build(&event_loop)
        .unwrap();
    std::thread::sleep(Duration::new(100, 0));
}
