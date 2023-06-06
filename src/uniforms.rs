use glam::{UVec2, Vec3};
use nannou;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    world: glam::Mat4,
    view: glam::Mat4,
    proj: glam::Mat4,
}

pub(crate) fn uniforms_as_bytes(uniforms: &Uniforms) -> &[u8] {
    unsafe { nannou::wgpu::bytes::from(uniforms) }
}

impl Uniforms {
    pub fn new(size: UVec2, view: glam::Mat4) -> Uniforms {
        let rotation = glam::Mat4::from_rotation_y(0f32);
        let aspect_ratio = size.x as f32 / size.y as f32;
        let fov_y = std::f32::consts::FRAC_PI_2;
        let near = 0.0001;
        let far = 100.0;
        let proj = glam::Mat4::perspective_rh_gl(fov_y, aspect_ratio, near, far);
        let scale = glam::Mat4::from_scale(Vec3::splat(0.01));
        Uniforms {
            world: rotation,
            view: (view * scale).into(),
            proj: proj.into(),
        }
    }
}