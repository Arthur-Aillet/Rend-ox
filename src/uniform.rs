use glam::{UVec2, Vec3};
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    world: glam::Mat4,
    view: glam::Mat4,
    proj: glam::Mat4,
}

fn uniforms_as_bytes(uniforms: &Uniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(uniforms) }
}

pub fn create_uniform_buffer(window_size : UVec2, view_matrix : glam::Mat4) -> wgpu::Buffer {
    let uniforms = create_uniforms(window_size, view_matrix);
    let uniforms_bytes = uniforms_as_bytes(&uniforms);
    let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
    device.create_buffer_init(&wgpu::BufferInitDescriptor {
        label: None,
        contents: uniforms_bytes,
    usage,
    })
}

fn create_uniforms(size: UVec2, view: glam::Mat4) -> Uniforms {
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
