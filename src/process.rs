use std::cell::{RefCell, RefMut};

use crate::app::App;
use crate::mesh::Mesh;
use crate::camera_controller::move_camera;
use crate::camera::Camera;
use crate::uniforms::Uniforms;
use crate::graphics::Graphics;

use nannou::event::{Event, Update};
use nannou::wgpu::IndexFormat::Uint16;
use nannou::wgpu::{self, BufferSize};
use nannou::winit;
use nannou::Frame;
use nannou_egui::{egui_wgpu_backend, Egui};

pub fn update<T>(nannou_app: &nannou::App, app: &mut App<T>, update: Update) {
    move_camera(nannou_app, app, &update);
    (app.user_update)(nannou_app, app, update);
}

pub fn event<T>(_nannou_app: &nannou::App, app: &mut App<T>, event: nannou::Event) {
    if app.camera_is_active {
        if let Event::DeviceEvent(_device_id, event) = event {
            if let winit::event::DeviceEvent::Motion { axis, value } = event {
                let sensitivity = 0.004;
                match axis {
                    // Yaw left and right on mouse x axis movement.
                    0 => app.camera.yaw -= (value * sensitivity) as f32,
                    // Pitch up and down on mouse y axis movement.
                    _ => {
                        let max_pitch = std::f32::consts::PI * 0.5 - 0.0001;
                        let min_pitch = -max_pitch;
                        app.camera.pitch = (app.camera.pitch + (-value * sensitivity) as f32)
                            .min(max_pitch)
                            .max(min_pitch)
                    }
                }
            } else if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                let sensitivity = 0.004;
                app.camera.yaw -= (delta.0 * sensitivity) as f32;
                let max_pitch = std::f32::consts::PI * 0.5 - 0.0001;
                let min_pitch = -max_pitch;
                app.camera.pitch = (app.camera.pitch + (-delta.1 * sensitivity) as f32)
                    .min(max_pitch)
                    .max(min_pitch);
            }
        }
    }
}

fn three_d_view_rendering(mut graphics : RefMut<Graphics>, frame: &Frame, mesh: &Mesh, camera: &Camera) {
    let depth_size = graphics.depth_texture.size();
    let device = frame.device_queue_pair().device();
    let frame_size = frame.texture_size();
    if frame_size != depth_size {
        let sample_count = frame.texture_msaa_samples();
        graphics.depth_texture = wgpu::TextureBuilder::new()
            .size(frame_size)
            .format(graphics.depth_texture.format())
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .sample_count(sample_count)
            .build(device);
        graphics.depth_texture_view = graphics.depth_texture.view().build();
    }

    // Update the uniforms
    let uniform_buffer = Uniforms::new_as_buffer_view(frame_size.into(), camera, device);
    let uniforms_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;

    let mut encoder = frame.command_encoder();
    encoder.copy_buffer_to_buffer(&uniform_buffer, 0, &graphics.uniform_buffer, 0, uniforms_size);

    let mut buffers: Vec<wgpu::Buffer> = vec![];
    let mut counts: Vec<usize> = vec![];
    graphics.draw(device, &mut buffers, &mut counts, mesh);
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        // We'll use a depth texture to assist with the order of rendering fragments based on depth.
        .depth_stencil_attachment(&graphics.depth_texture_view, |depth| depth)
        .begin(&mut encoder);
    render_pass.set_bind_group(0, &graphics.bind_group, &[]);
    render_pass.set_pipeline(&graphics.render_pipeline);

    let mut count = counts.iter();
    for i in (0..buffers.len()).step_by(4) {
        render_pass.set_index_buffer(buffers[i].slice(..), Uint16);
        render_pass.set_vertex_buffer(0, buffers[i + 1].slice(..));
        render_pass.set_vertex_buffer(1, buffers[i + 2].slice(..));
        render_pass.set_vertex_buffer(2, buffers[i + 3].slice(..));
        if let Some(c) = count.next() {
            render_pass.draw_indexed(0..*c as u32, 0, 0..1);
        }
    }
}

pub fn view<T>(_nannou_app: &nannou::App, app: &App<T>, frame: Frame) {
    let mut graphics = app.graphics.borrow_mut();

    three_d_view_rendering(graphics, &frame, &app.mesh, &app.camera);
    app.egui_instance.draw_to_frame(&frame);
}
