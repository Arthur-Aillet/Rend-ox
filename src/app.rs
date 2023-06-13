use std::cell::RefCell;
use std::ops::Deref;

use crate::Vec3;
use nannou;
use nannou::wgpu;
use nannou_egui::Egui;
use glam::Mat4;

use crate::camera_controller::key_pressed;
use crate::graphics::Graphics;
use crate::mesh::{Mesh, MeshDescriptor};
use crate::process::{view, event, update};

pub struct App<T> {
    pub camera_is_active: bool,
    pub graphics: RefCell<Graphics>,
    pub camera: crate::camera::Camera,
    // pub mesh: MeshDescriptor,
    pub egui_instance: Egui,
    pub user: T,
    pub user_update: UpdateFn<T>
}

pub fn vertices_as_bytes_copy(data: &Vec<Vec3>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        for i in 0..3 {
            final_bytes.extend(elem[i].to_le_bytes());
        }
    }
    final_bytes
}

pub fn indices_as_bytes_copy(data: &Vec<u16>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        final_bytes.push(*elem as u8);
        final_bytes.push((*elem >> 8) as u8);
    }
    final_bytes
}

pub(crate) fn matrices_as_bytes_copy(data: &Vec<Mat4>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        for i in elem.to_cols_array() {
            final_bytes.extend(i.to_le_bytes());
        }
    }
    final_bytes
}

pub fn launch_rendox_app<T: 'static>(model: RendoxAppFn<T>) {
    nannou::app(model).event(event).update(update).run();
}

pub type RendoxAppFn<T> = fn(_: &nannou::App) -> App<T>;
pub type UpdateFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: crate::nannou::event::Update);

pub fn app<T: 'static>(nannou_app: &nannou::App, user: T, user_update: UpdateFn<T>) -> App<T> {
    match create_app(nannou_app, user, user_update) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Failed to create Model: {err}");
            std::process::exit(84);
        }
    }
}

fn raw_window_event<T>(_app: &nannou::App, model: &mut App<T>, event: &nannou::winit::event::WindowEvent) {
    model.egui_instance.handle_raw_event(event);
}
fn create_app<T: 'static>(
    nannou_app: &nannou::App,
    user: T,
    user_update: UpdateFn<T>
) -> Result<App<T>, Box<dyn std::error::Error>> {
    let w_id = match nannou_app
        .new_window()
        .size(1024, 576)
        .key_pressed::<App<T>>(key_pressed)
        .raw_event(raw_window_event::<T>)
        .view::<App<T>>(view)
        .build()
    {
        Ok(val) => val,
        Err(_err) => {
            return Err(Box::new(crate::error::RendError::new(
                "Window Builder failed",
            )))
        }
    };

    let window = match nannou_app.window(w_id) {
        None => {
            return Err(Box::new(crate::error::RendError::new(
                "Invalid window id found",
            )))
        }
        Some(val) => val,
    };

    let egui_instance = Egui::from_window(&window);


    let camera_is_active = true;
    match window.set_cursor_grab(true) {
        Err(_err) => {
            return Err(Box::new(crate::error::RendError::new(
                "Cursor can't be grabbed",
            )))
        }
        _ => {}
    }
    window.set_cursor_visible(false);

    let camera = crate::camera::Camera::new();


    println!("Use the `W`, `A`, `S`, `D`, `Q` and `E` keys to move the camera.");
    println!("Use the mouse to orient the pitch and yaw of the camera.");
    println!("Press the `Space` key to toggle camera mode.");

    // let ret = Mesh::from_obj("./.objs/bat.obj");

    let mut graphics = Graphics::create(window.deref(), &camera);
    // let ret = graphics.load_mesh("./.objs/bat.obj");
    // match ret {
    //     Err(e) => return Err(e),
    //     Ok(mesh) => {
            Ok(App {
                camera_is_active,
                graphics: RefCell::new(graphics),
                camera,
                // mesh,
                user,
                user_update,
                egui_instance,
            })
        // }
    // }
}

impl<T> App<T> {
    pub fn draw(&self, md: &MeshDescriptor) -> bool {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            if let Some(old) = g.draw_queue.get_mut(&md) {
                old.append(&mut vec![Mat4::IDENTITY]);
            } else {
                g.draw_queue.insert(md.clone(), vec![Mat4::IDENTITY]);
            }
            return true;
        }
        println!("Rendox: failed draw call of {}", md.name);
        false
    }
    pub fn draw_instances(&self, md: &MeshDescriptor, mut instances: Vec<Mat4>) -> bool {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            if let Some(old) = g.draw_queue.get_mut(&md) {
                old.append(&mut instances);
            } else {
                g.draw_queue.insert(md.clone(), instances);
            }
            return true;
        }
        println!("Rendox: failed instanced draw call of {}", md.name);
        false
    }
}
