use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;

use crate::Vec3;
use glam::{EulerRot, Mat4, Quat};
use nannou;
use nannou_egui::Egui;
use nannou_egui::egui::CtxRef;

use crate::camera_controller::key_pressed;
use crate::graphics::Graphics;
use crate::mesh::MeshDescriptor;
use crate::process::{event, update, view};

pub struct App<T> {
    pub camera_is_active: bool,
    pub graphics: RefCell<Graphics>,
    pub camera: crate::camera::Camera,
    // pub mesh: MeshDescriptor,
    pub(crate) egui_instance: Rc<RefCell<Egui>>,
    pub user: T,
    pub user_update: Option<UpdateFn<T>>,
    pub user_keypressed: Option<UserKeyPressedFn<T>>,
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
pub type UpdateFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: crate::nannou::event::Update, _: &CtxRef);
pub type UserKeyPressedFn<T> = fn(_: &nannou::App, _: &mut App<T>, _: crate::nannou::event::Key);

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

    println!("Use the `W`, `A`, `S`, `D`, `Q` and `E` keys to move the camera.");
    println!("Use the mouse to orient the pitch and yaw of the camera.");
    println!("Press the `Space` key to toggle camera mode.");

    // let ret = Mesh::from_obj("./.objs/bat.obj");

    let graphics = Graphics::create(window.deref(), &camera);
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
        user_update: None,
        egui_instance,
        user_keypressed: None,
    })
    // }
    // }
}

impl<T> App<T> {
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
