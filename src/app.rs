use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::graphics::Graphics;
use crate::model::Model;
use crate::window::{State};
use pollster;

pub struct App {
    event_loop : EventLoop<T>,
    graphics : Graphics,
    model : Model,
    state : State,
    window : Window,
    update : Option<fn(&App)>
}

impl App {
    pub fn run() {
        pollster::block_on(async_run())
    }

    fn update(&mut self, update : fn(&App)) {
        self.update = Some(update);
    }

    async fn async_run() {
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

    pub async fn new(create_model : fn()->Model) -> Result<App, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop);
        let window = match window {
            Ok(model) => model,
            Err(err) => {
                eprintln!("Failed to create Window: {err}");
                std::process::exit(84)
            }
        };
        let graphics = Graphics::new(&window);
        let model = create_model();
        let state = State::new(&window).await;

        Ok(App {
            event_loop,
            graphics,
            model,
            window,
            state,
            update : None,
        })
    }
}


