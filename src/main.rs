use vulkano::{
    instance::{Instance, PhysicalDevice},
    device::{Device},
};


use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::dpi::{PhysicalPosition, Pixel};

use cgmath::{Deg, Rad, Euler, Angle};


use crate::renderer::Render;
use crate::ui::Widget;
use crate::ui::text::Text;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::time::Instant;

mod mesh;
mod ui;
// mod hud;

mod renderer;
mod texture;
mod chunk;
mod world;
mod block;
mod camera;
mod terrain;
mod datatypes;


fn main() {
    println!("PROGRAM - BEGIN INITIALIZATION");
    let mut maximized = false;

    // setup
    let instance= {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create instance")
    };

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&event_loop, instance.clone()).unwrap();

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()
        };
        Device::new(physical, physical.supported_features(), &device_ext,
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    println!("PROGRAM - BEGIN MAIN PROGRAM");

    use winit::event_loop::ControlFlow;
    use winit::event::{Event, WindowEvent, DeviceEvent, VirtualKeyCode as K, KeyboardInput, ElementState};
    use winit::dpi::Position;

    let mut rotation = Euler::new(Deg(0.0 as f32), Deg(0.0), Deg(0.0));  // the rotation of the player's camera in Radian
    let mut pressed: Vec<K> = Vec::new();  // keyboard pressed for player translation
    let mut cmd_mode = false;  // command/chat mode to use commands/chat/or simply exit the mouse lock state TODO: temporary; we'll be using a special struct to handle states and inputs
    // TODO: a global pushdown state-machine stack to create an pause menu
    // TODO: also use cgmath's position and etc. for actual position to stay consistent

    let mut render = Render::new(physical.clone(), device.clone(),queue.clone(), surface.clone());

    let txt = render.ui.add_widget(Text::new("FPS: 0".into(), [-0.9, -0.9], 0.01));
    println!("PROGRAM - START MAIN LOOP");

    let mut frames = 0;
    let mut start = Instant::now();

    event_loop.run( move |event, _, control_flow| {
        frames += 1;

        let duration = start.elapsed().as_secs_f64();
        if duration >= 1.0 {  // greater than 1 second
            (*txt).borrow_mut().text = format!("FPS: {}", (frames as f64*(1.0/duration)).round() as u32);

            frames = 0;
            start = Instant::now();
        }

        let dimensions: [u32; 2] = surface.window().inner_size().into();
        render.ui.update(&event);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {*control_flow = ControlFlow::Exit},
                    WindowEvent::KeyboardInput { input, ..} => {
                        match input {
                            KeyboardInput { virtual_keycode: key, state: ElementState::Pressed, ..} => {
                                match key.unwrap() {
                                    K::Escape => {*control_flow = ControlFlow::Exit},
                                    K::F11 => {
                                        maximized = !maximized;
                                        surface.window().set_maximized(maximized);
                                    },
                                    K::T => {cmd_mode = !cmd_mode},
                                    K::A => { if !pressed.contains(&K::A) {pressed.push(K::A);} },
                                    K::D => { if !pressed.contains(&K::D) {pressed.push(K::D);} },
                                    K::W => { if !pressed.contains(&K::W) {pressed.push(K::W);} },
                                    K::S => { if !pressed.contains(&K::D) {pressed.push(K::S);} },
                                    K::LShift => { if !pressed.contains(&K::LShift) {pressed.push(K::LShift);} },
                                    K::Space =>  { if !pressed.contains(&K::Space) {pressed.push(K::Space);} },
                                    _ => {}
                                }
                            },
                            KeyboardInput { virtual_keycode: key, state: ElementState::Released, ..} => {
                                match key.unwrap() {
                                    K::A => { if pressed.contains(&K::A) {pressed.retain(|i| i != &K::A);} },
                                    K::D => { if pressed.contains(&K::D) {pressed.retain(|i| i != &K::D);} },
                                    K::W => { if pressed.contains(&K::W) {pressed.retain(|i| i != &K::W);} },
                                    K::S => { if pressed.contains(&K::S) {pressed.retain(|i| i != &K::S);} },
                                    K::LShift => { if pressed.contains(&K::LShift) {pressed.retain(|i| i != &K::LShift);} },
                                    K::Space => { if pressed.contains(&K::Space) {pressed.retain(|i| i != &K::Space);} },
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // println!("{} {}", delta.0, delta.1);
                if !cmd_mode {
                    rotation.x -= Deg(delta.1 as f32/10.0);
                    rotation.y += Deg(delta.0 as f32/10.0);
                    render.cam.rotate(delta.1 as f32, delta.0 as f32);

                    surface.window().set_cursor_position(
                        Position::Physical(PhysicalPosition{ x: dimensions[0] as i32/2, y: dimensions[1] as i32/2 })
                    ).unwrap();
                }
            },
            // this calls last after all the event finishes emitting
            // and only calls once, which is great for updating mutable variables since it'll be uniform
            Event::MainEventsCleared => {
                if pressed.contains(&K::A) {render.cam.translate(-Rad(rotation.y).0.cos(), 0.0, Rad(rotation.y).0.sin())}
                if pressed.contains(&K::D) {render.cam.translate(Rad(rotation.y).0.cos(), 0.0, -Rad(rotation.y).0.sin())}
                if pressed.contains(&K::W) {render.cam.translate(Rad(rotation.y).0.sin(), 0.0, Rad(rotation.y).0.cos())}
                if pressed.contains(&K::S) {render.cam.translate(-Rad(rotation.y).0.sin(),0.0, -Rad(rotation.y).0.cos())}
                if pressed.contains(&K::LShift) {render.cam.translate(0.0, -1.0, 0.0)}
                if pressed.contains(&K::Space)  {render.cam.translate(0.0, 1.0, 0.0)}

            },
            Event::RedrawEventsCleared => {
                render.update(device.clone(), queue.clone(), dimensions);
            },
            _ => {},
        }
    });
}
