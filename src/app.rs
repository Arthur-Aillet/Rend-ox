//! Main app
//! Stores everything persistent to the engine, and a custom type for users,
//! exposes methods for the graphics module
//! it is created using nannou, but that dependency is planned to be removed

use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use crate::Vec3;
use glam::{EulerRot, Mat4, Quat};
use nannou;
use nannou_egui::Egui;
use nannou_egui::egui::CtxRef;

use crate::camera_controller::key_pressed;
use crate::graphics::{Graphics, MaterialSlot, ShaderSlot};
use crate::material::{MaterialDescriptor};
use crate::mesh::MeshDescriptor;
use crate::error::RendError;
use crate::process::{event, update, view};

pub type RendoxAppFn<T> = fn(_: &nannou::App) -> App<T>;
pub type UpdateFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: crate::nannou::event::Update, _: &CtxRef);
pub type UserKeyPressedFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: crate::nannou::event::Key);
pub type EventFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: nannou::Event);

pub struct App<T> {
    pub camera_is_active: bool,
    pub(crate) graphics: RefCell<Graphics>,
    pub camera: crate::camera::Camera,
    // pub mesh: MeshDescriptor,
    pub(crate) egui_instance: Rc<RefCell<Egui>>,
    pub user: T,
    pub user_update: Option<UpdateFn<T>>,
    pub user_keypressed: Option<UserKeyPressedFn<T>>,
    pub user_event: Option<EventFn<T>>,
}

pub(crate) fn vertices_as_bytes_copy(data: &Vec<Vec3>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        for i in 0..3 {
            final_bytes.extend(elem[i].to_le_bytes());
        }
    }
    final_bytes
}

pub(crate) fn indices_as_bytes_copy(data: &Vec<u16>) -> Vec<u8> {
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

pub fn app<T: 'static>(nannou_app: &nannou::App, user: T) -> App<T> {
    match create_app(nannou_app, user) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Failed to create Model: {err}");
            std::process::exit(84);
        }
    }
}

impl<T> App<T> {
    pub fn update(mut self, user_update: UpdateFn<T>) -> App<T> {
        self.user_update = Some(user_update);
        self
    }

    pub fn key_pressed(mut self, user_key_pressed: UserKeyPressedFn<T>) -> App<T> {
        self.user_keypressed = Some(user_key_pressed);
        self
    }
    pub fn event(mut self, user_event: EventFn<T>) -> App<T> {
        self.user_event = Some(user_event);
        self
    }
}

fn raw_window_event<T>(
    _app: &nannou::App,
    model: &mut App<T>,
    event: &nannou::winit::event::WindowEvent,
) {
    if let Ok(mut egui) = model.egui_instance.try_borrow_mut() {
        egui.handle_raw_event(event);
    }
}

fn create_app<T: 'static>(
    nannou_app: &nannou::App,
    user: T,
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

    let egui_instance = Rc::new(RefCell::new(Egui::from_window(&window)));

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

    let graphics = Graphics::create(window.deref(), &camera);

    Ok(App {
        camera_is_active,
        graphics: RefCell::new(graphics),
        camera,
        user,
        egui_instance,
        user_update: None,
        user_keypressed: None,
        user_event: None,
    })
}

impl<T> App<T> {
    /// load a fragment shader.
    ///
    /// the format must be identical to fs.wgsl or wgpu will panic
    pub fn load_shader(&mut self, path: &str) -> Result<ShaderSlot, Box<dyn std::error::Error>> {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            return g.load_shader(path);
        }
        return Err(Box::new(RendError::new("Graphics module borrowed")));
    }

    /// load a material, its textures and associated shader
    pub fn load_material(&mut self, material: MaterialDescriptor) -> Result<MaterialSlot, Box<dyn std::error::Error>> {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            return Ok(g.load_material(material));
        }
        return Err(Box::new(RendError::new("Graphics module borrowed")));
    }

    /// set a MeshDescriptor to use a given material for following draw calls
    pub fn bind_material_to_mesh(&self, md: &mut MeshDescriptor, material: &MaterialSlot) -> bool {
        if let Ok(g) = self.graphics.try_borrow() {
            return g.bind_material_to_mesh(md, material);
        }
        false
    }

    /// load a mesh from a file and return a unique MeshDescriptor
    pub fn load_mesh(&mut self, path: &str) -> Result<MeshDescriptor, Box<dyn std::error::Error>> {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            return g.load_mesh(path);
        }
        return Err(Box::new(RendError::new("Graphics module borrowed")));
    }

    /// draw a mesh with no transforms
    pub fn draw(&self, md: &MeshDescriptor, color: Vec3) -> bool {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            if let Some((old_col, old_inst)) = g.draw_queue.get_mut(&md) {
                old_col.append(&mut vec![color]);
                old_inst.append(&mut vec![Mat4::IDENTITY]);
            } else {
                g.draw_queue
                    .insert(md.clone(), (vec![color], vec![Mat4::IDENTITY]));
            }
            return true;
        }
        println!("Rendox: failed draw call of {}", md.name);
        false
    }

    /// draw a mesh at a given position rotation and scale, with given instance color
    pub fn draw_at(&self, md: &MeshDescriptor, color: Vec3, pos: Vec3, rot : Vec3, scale : Vec3) -> bool {
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            if let Some((old_col, old_inst)) = g.draw_queue.get_mut(&md) {
                old_col.append(&mut vec![color]);
                old_inst.append(&mut vec![Mat4::from_scale_rotation_translation(scale, Quat::from_euler(EulerRot::XYZ, rot.x, rot.y, rot.z), pos)]);
            } else {
                g.draw_queue
                    .insert(md.clone(), (vec![color], vec![Mat4::from_scale_rotation_translation(scale, Quat::from_euler(EulerRot::XYZ, rot.x, rot.y, rot.z), pos)]));
            }
            return true;
        }
        println!("Rendox: failed draw call of {}", md.name);
        false
    }

    /// draw a single mesh multiple times with different transforms
    /// each instance is given by a matrix transformation
    /// and colors loop over, with a default of white
    pub fn draw_instances(
        &self,
        md: &MeshDescriptor,
        mut instances: Vec<Mat4>,
        colors: Vec<Vec3>
    ) -> bool {
        let mut c = match colors.len() {
            0 => vec![Vec3::new(1., 1., 1.)],
            _ => colors
        }.iter().cloned().cycle().take(instances.len()).collect();
        if let Ok(mut g) = self.graphics.try_borrow_mut() {
            if let Some((old_col, old_inst)) = g.draw_queue.get_mut(&md) {
                old_col.append(&mut c);
                old_inst.append(&mut instances);
            } else {
                g.draw_queue.insert(md.clone(), (c, instances));
            }
            return true;
        }
        println!("Rendox: failed instanced draw call of {}", md.name);
        false
    }
}
