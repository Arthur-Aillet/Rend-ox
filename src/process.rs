use crate::app::App;
use crate::camera_controller::move_camera;
use crate::uniforms::Uniforms;

use nannou::event::{Event, Update};
use nannou::wgpu;
use nannou::winit;
use nannou::Frame;

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

pub fn view<T>(_nannou_app: &nannou::App, app: &App<T>, frame: Frame) {
    let mut g = app.graphics.borrow_mut();

    // If the window has changed size, recreate our depth texture to match.
    let depth_size = g.depth_texture.size();
    let device = frame.device_queue_pair().device();
    let frame_size = frame.texture_size();
    if frame_size != depth_size {
        let sample_count = frame.texture_msaa_samples();
        g.depth_texture = wgpu::TextureBuilder::new()
            .size(frame_size)
            .format(g.depth_texture.format())
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .sample_count(sample_count)
            .build(device);
        g.depth_texture_view = g.depth_texture.view().build();
    }

    // Update the uniforms
    let uniform_buffer = Uniforms::new_as_buffer_view(frame_size.into(), &app.camera, device);
    let uniforms_size = std::mem::size_of::<Uniforms>() as wgpu::BufferAddress;

    let mut encoder = frame.command_encoder();
    encoder.copy_buffer_to_buffer(&uniform_buffer, 0, &g.uniform_buffer, 0, uniforms_size);

    let mut buffers = vec![] as Vec<wgpu::Buffer>;
    let mut counts = vec![] as Vec<usize>;
    g.draw(device, &mut buffers, &mut counts, &app.mesh);
    {
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(frame.texture_view(), |color| color)
            // We'll use a depth texture to assist with the order of rendering fragments based on depth.
            .depth_stencil_attachment(&g.depth_texture_view, |depth| depth)
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &g.bind_group, &[]);
        render_pass.set_pipeline(&g.render_pipeline);

        let mut i = 0;
        let mut count = counts.iter();
        while i < buffers.len() {
            render_pass.set_index_buffer(buffers[i].slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(0, buffers[i + 1].slice(..));
            render_pass.set_vertex_buffer(1, buffers[i + 2].slice(..));
            render_pass.set_vertex_buffer(2, buffers[i + 3].slice(..));
            if let Some(c) = count.next() {
                render_pass.draw_indexed(0..*c as u32, 0, 0..1);
            }

            i += 4;
        }
    }
    // g.render_pass = Some(render_pass);
    //     render_pass.set_index_buffer(g.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //     render_pass.set_vertex_buffer(0, g.vertex_buffer.slice(..));
    //     render_pass.set_vertex_buffer(1, g.uv_buffer.slice(..));
    //     render_pass.set_vertex_buffer(2, g.normal_buffer.slice(..));
    //     render_pass.draw_indexed(0..g.index_count as u32, 0, 0..1);
}
