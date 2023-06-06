use std::cell::RefCell;
use crate::app::App;
use crate::graphics::Graphics;
use crate::obj::{Indices, Mesh, Normals, Vertices};
use crate::uniform::{create_uniform_buffer, Uniforms};

pub struct Model {
    pub camera_is_active: bool,
    pub camera: crate::camera::Camera,
    pub _mesh: Mesh,
    pub buffers: (Indices, Vertices, Vertices, Normals),
}

fn create_model() -> Result<Model, Box<dyn std::error::Error>> {
    let camera_is_active = true;
    let window_size: glam::UVec2 = window.inner_size_pixels().into();


    let mut mesh: Mesh = Mesh::new();
    if !mesh.parse_obj("./.objs/bat.obj") {
        return Err(Box::new(rend_ox::error::RendError::new(
            "Invalid or non supported obj file!",
        )));
    }
    let camera = rend_ox::camera::Camera::new();

    let buffers = mesh.as_buffers();

    println!("Use the `W`, `A`, `S`, `D`, `Q` and `E` keys to move the camera.");
    println!("Use the mouse to orient the pitch and yaw of the camera.");
    println!("Press the `Space` key to toggle camera mode.");

    Ok(Model {
        camera_is_active,
        camera,
        _mesh: mesh,
        buffers,
    })
}
impl Model {
    pub fn new() -> Model {
        match create_model() {
            Ok(model) => model,
            Err(err) => {
                eprintln!("Failed to create Model: {err}");
                std::process::exit(84)
            }
        }
    }
}