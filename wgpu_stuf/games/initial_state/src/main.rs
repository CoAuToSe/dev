#![allow(unused_variables)]
use std::time::Duration;

use winit::{
    self,
    event::{DeviceEvent, Event},
    event_loop::ControlFlow,
    *,
};

#[derive(Debug)]
enum CustomEvent<'a> {
    Som(winit::event::Event<'a, DeviceEvent>),
    Non,
}

// pub struct DeviceId(u32);
fn main() {
    println!("Hello, world!");
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
                window_id: my_window.id(),
                event: winit::event::WindowEvent::Focused(true),
            }))
            .ok();
    });

    event_loop.run(move |event, something, control_flow| {
        println!("{:?} {:?} {:?}", event, something, control_flow);
        match event {
            Event::NewEvents(some) => (),
            Event::WindowEvent { window_id, event } => match event {
                event::WindowEvent::Resized(some) => (),
                event::WindowEvent::Moved(some) => (),
                event::WindowEvent::CloseRequested => (),
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
                } => (),
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
                } => (),
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
            Event::UserEvent(some) => (),
            Event::Suspended => (),
            Event::Resumed => (),
            Event::MainEventsCleared => (),
            Event::RedrawRequested(some) => (),
            Event::RedrawEventsCleared => (),
            Event::LoopDestroyed => (),
        }
        // *control_flow = ControlFlow::Poll;
        *control_flow = ControlFlow::Wait;
        // *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10000));
        // *control_flow = ControlFlow::Exit;
    });
}
