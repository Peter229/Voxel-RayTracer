#![allow(unused_variables)]
#![allow(unused_imports)]

mod texture;
mod r_state;
mod r_pipeline;
mod camera;
mod uniform;
mod game;
mod input;
mod chunk;
mod svo;
mod level;

//Game tick every 16 ms
const NUM_TICKS: u128 = 16;
const EVERY_SECOND: u128 = 1000;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

use futures::executor::block_on;
use std::time::Instant;

fn main() {

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("VX");
    window.set_inner_size(winit::dpi::LogicalSize::new(1280, 720));
    //window.set_cursor_grab(true).unwrap();
    //window.set_cursor_visible(true);
    let mut r_state = block_on(r_state::State::new(&window));

    let mut fps: i32 = 0;
    let mut run_time = Instant::now();
    let mut tick_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::DeviceEvent {
                ref event,
                ..
            } => {
                r_state.raw_mouse(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                r_state.input(event);
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        r_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        r_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                let delta_time = (((run_time.elapsed().as_nanos() as f64 / 1000.0) / 1000.0) / 1000.0) as f32;
                run_time = Instant::now();
                r_state.update(delta_time);
                //println!("{}", 1.0 / delta_time);
                //game.process_inputs([r_state.size.width, r_state.size.height]);
                //fps += 1;
                //if run_time.elapsed().as_millis() >= EVERY_SECOND {
                    //println!("fps {}", fps);
                    //fps = 0;
                    //run_time = Instant::now();
                //}
                if tick_time.elapsed().as_millis() >= NUM_TICKS {
                    tick_time = Instant::now();
                    r_state.render();
                }
            }
            _ => {}
        }
    });
}
