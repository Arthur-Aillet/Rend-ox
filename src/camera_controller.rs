use crate::app::App;

use nannou;
use nannou::event::{Key, Update};
use nannou::time::DurationF64;

pub(crate) fn key_pressed<T>(app: &nannou::App, model: &mut App<T>, key: Key) {
    if let Key::Space = key {
        let window = app.main_window();
        if !model.camera_is_active {
            if window.set_cursor_grab(true).is_ok() {
                model.camera_is_active = true;
            }
        } else {
            if window.set_cursor_grab(false).is_ok() {
                model.camera_is_active = false;
            }
        }
        window.set_cursor_visible(!model.camera_is_active);
    }
}

pub fn default_camera<T>(app: &nannou::App, model: &mut App<T>, update: &Update) {
    if model.camera_is_active {
        let velocity = (update.since_last.secs() * model.camera.speed / 10.) as f32;

        if app.keys.down.contains(&Key::Z) {
            model.camera.move_forward(velocity);
        }

        if app.keys.down.contains(&Key::S) {
            model.camera.move_forward(-velocity);
        }

        if app.keys.down.contains(&Key::Q) {
            model.camera.move_right(-velocity)
        }

        if app.keys.down.contains(&Key::D) {
            model.camera.move_right(velocity)
        }

        if app.keys.down.contains(&Key::A) {
            model.camera.move_up(-velocity)
        }

        if app.keys.down.contains(&Key::E) {
            model.camera.move_up(velocity)
        }
    }
}
