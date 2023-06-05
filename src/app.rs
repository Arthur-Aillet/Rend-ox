use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::graphics::Graphics;
use crate::model::Model;
use crate::window::State;
use pollster;

pub struct App {
    event_loop : EventLoop<T>,
    graphics : Graphics,
    model : Model,
    state : State,
    window : Window,
}

impl App {
    pub fn run() {
        pollster::block_on(async_run());
    }

    pub async fn async_run() {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            },
            _ => {}
        });
    }

    pub async fn new(graphics : Graphics, model : Model) -> App {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let mut state = State::new(&window).await.unwrap();
        App {
            event_loop,
            graphics,
            model,
            window,
            state,
        }
    }
}


