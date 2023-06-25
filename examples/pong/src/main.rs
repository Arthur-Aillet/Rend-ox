use rend_ox::app::{app, App};
use rend_ox::mesh::MeshDescriptor;
use rend_ox::nannou::event::Key;
use rend_ox::nannou_egui::egui::CtxRef;
use rend_ox::Vec3;
use std::f64::consts::PI;

pub struct Pong {
    pub ball: Option<MeshDescriptor>,
    pub rack: Option<MeshDescriptor>,
    pub fst_height: f32,
    pub snd_height: f32,
    pub speed: f64,
    pub angle: f64,
    pub ball_pos: Vec3,
}

impl Pong {
    fn new() -> Pong {
        Pong {
            ball: None,
            rack: None,
            fst_height: 0.0,
            snd_height: 0.0,
            speed: 0.1,
            angle: 0.,
            ball_pos: Vec3::new(0.0, -30.0, 0.0),
        }
    }
}

fn bound_val(val: f32, min: f32, max: f32) -> f32 {
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }
    val
}

pub fn pong_update(
    nannou: &rend_ox::nannou::App,
    app: &mut App<Pong>,
    _update: rend_ox::nannou::event::Update,
    _: &CtxRef,
) {
    app.user.speed +=
        app.user.speed * 0.0001 * nannou.duration.since_prev_update.as_millis() as f64;
    if nannou.keys.down.contains(&Key::T) {
        app.user.fst_height += 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    if nannou.keys.down.contains(&Key::G) {
        app.user.fst_height -= 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    app.user.fst_height = bound_val(app.user.fst_height, -17., 17.);

    if nannou.keys.down.contains(&Key::O) {
        app.user.snd_height += 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    if nannou.keys.down.contains(&Key::L) {
        app.user.snd_height -= 0.025 * nannou.duration.since_prev_update.as_millis() as f32;
    }
    app.user.snd_height = bound_val(app.user.snd_height, -17., 17.);

    let movement = Vec3::new(
        app.user.speed as f32 * app.user.angle.cos() as f32,
        0.,
        app.user.speed as f32 * app.user.angle.sin() as f32,
    );
    app.user.ball_pos = app.user.ball_pos + movement;

    if (app.user.ball_pos.x > 26. && (app.user.fst_height - app.user.ball_pos.z).abs() > 8.)
        || (app.user.ball_pos.x < -26. && (app.user.snd_height - app.user.ball_pos.z).abs() > 8.)
    {
        app.user.speed = 0.1;
        app.user.angle = PI;
        app.user.ball_pos = Vec3::new(0.0, -30.0, 0.0);
    }

    if app.user.ball_pos.x > 26. {
        app.user.angle = app.user.angle - PI
            + (((app.user.fst_height - app.user.ball_pos.z) / 8.0) as f64 * PI / 4.0) as f64;
    }

    if app.user.ball_pos.x < -26. {
        app.user.angle = app.user.angle
            - PI
            - (((app.user.snd_height - app.user.ball_pos.z) / 8.0) as f64 * PI / 4.0) as f64;
    }

    if app.user.ball_pos.z > 13. || app.user.ball_pos.z < -13. {
        app.user.angle = (PI * 2.0) - app.user.angle;
    }

    if let Some(mesh) = &app.user.ball {
        app.draw_at(
            mesh,
            Vec3::new(0.2, 1., 12.),
            app.user.ball_pos,
            Vec3::new(0., 0., 0.),
            Vec3::new(2., 2., 2.),
        );
    }
    if let Some(mesh) = &app.user.rack {
        app.draw_at(
            mesh,
            Vec3::new(1., 1., 12.),
            Vec3::new(30., -30., app.user.fst_height),
            Vec3::new(0., std::f32::consts::PI / 2., std::f32::consts::PI / 2.),
            Vec3::new(2., 2., 3.),
        );
        app.draw_at(
            mesh,
            Vec3::new(1., 1., 12.),
            Vec3::new(-30., -30., app.user.snd_height),
            Vec3::new(0., -std::f32::consts::PI / 2., std::f32::consts::PI / 2.),
            Vec3::new(2., 2., 3.),
        );
    }
}

fn pong_app(nannou_app: &rend_ox::nannou::App) -> App<Pong> {
    let mut app = app(nannou_app, Pong::new()).update(pong_update);

    if let Ok(md) = app.load_mesh("./ball.obj") {
        app.user.ball = Some(md);
    } else {
        println!("Error loading ball!")
    }
    if let Ok(md) = app.load_mesh("./rack.obj") {
        app.user.rack = Some(md);
    } else {
        println!("Error loading rack!")
    }
    app
}

fn main() {
    rend_ox::app::launch_rendox_app(pong_app);
}
