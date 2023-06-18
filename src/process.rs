use std::cell::RefMut;

use crate::app::{matrices_as_bytes_copy, vertices_as_bytes_copy, App};
use crate::camera::Camera;
use crate::camera_controller::move_camera;
use crate::graphics::Graphics;
use crate::uniforms::Uniforms;
use glam::Mat4;

use nannou::event::{Event, Update};
use nannou::wgpu;
use nannou::wgpu::util::DeviceExt;
use nannou::winit;
use nannou::Frame;

pub fn update<T>(nannou_app: &nannou::App, app: &mut App<T>, update: Update) {
    move_camera(nannou_app, app, &update);
    (app.user_update)(nannou_app, app, update);
    // {
    //     app.draw(&app.mesh);
    // }
}

pub fn event<T>(_nannou_app: &nannou::App, app: &mut App<T>, event: nannou::Event) {
    if app.camera_is_active {
        if let Event::DeviceEvent(_device_id, event) = event {
            if let winit::event::DeviceEvent::Motion { axis, value } = event {
                let sensitivity = app.camera.sensitivity / 1000.;
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
                let sensitivity = app.camera.sensitivity / 1000.;
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

fn three_d_view_rendering(mut graphics: RefMut<Graphics>, frame: &Frame, camera: &Camera) {
    let depth_size = graphics.depth_texture.size();
    let device = frame.device_queue_pair().device();
    graphics.refresh_shaders(device);
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
    encoder.copy_buffer_to_buffer(
        &uniform_buffer,
        0,
        &graphics.uniform_buffer,
        0,
        uniforms_size,
    );

    let mut buffers: Vec<wgpu::Buffer> = vec![];
    let mut shaders: Vec<usize> = vec![];
    let mut instance_buffers: Vec<wgpu::Buffer> = vec![];
    let mut inst_color_buffers: Vec<wgpu::Buffer> = vec![];
    let mut counts: Vec<usize> = vec![];
    let mut all_instances: Vec<Vec<Mat4>> = vec![]; //= vec![Mat4::from_rotation_x(std::f32::consts::PI * 0.5), Mat4::from_translation(Vec3::new(2., 0., 0.))];

    for (md, (colors, instances)) in &graphics.draw_queue {
        if let Some(mesh) = graphics.meshes.get(&md.idx) {
            graphics.draw(device, &mut buffers, &mut counts, mesh);
            shaders.push(md.shader);
            all_instances.push(instances.clone());
            let raw_instance_col = vertices_as_bytes_copy(colors);
            let raw_instance_mat = matrices_as_bytes_copy(instances);
            inst_color_buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
                label: None,
                contents: &*raw_instance_col,
                usage: wgpu::BufferUsages::VERTEX,
            }));
            instance_buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
                label: None,
                contents: &*raw_instance_mat,
                usage: wgpu::BufferUsages::VERTEX,
            }));
        }
    }
    graphics.draw_queue.clear();
    {
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(frame.texture_view(), |color| color)
            // We'll use a depth texture to assist with the order of rendering fragments based on depth.
            .depth_stencil_attachment(&graphics.depth_texture_view, |depth| depth)
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &graphics.bind_group, &[]);

        let mut count = counts.iter();
        let mut shader = shaders.iter();
        let mut instance = all_instances.iter();
        let mut instance_buffer = instance_buffers.iter();
        let mut instance_color = inst_color_buffers.iter();
        for i in (0..buffers.len()).step_by(4) {
            if let (Some(c), Some(inst), Some(inst_buff), Some(inst_color), Some(s)) = (
                count.next(),
                instance.next(),
                instance_buffer.next(),
                instance_color.next(),
                shader.next(),
            ) {
                render_pass.set_pipeline(&graphics.render_pipelines[s]);
                render_pass.set_index_buffer(buffers[i].slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_vertex_buffer(0, buffers[i + 1].slice(..));
                render_pass.set_vertex_buffer(1, buffers[i + 2].slice(..));
                render_pass.set_vertex_buffer(2, buffers[i + 3].slice(..));
                render_pass.set_vertex_buffer(3, inst_color.slice(..));
                render_pass.set_vertex_buffer(4, inst_buff.slice(..));
                render_pass.draw_indexed(0..*c as u32, 0, 0..inst.len() as u32);
            }
        }
    }
}

pub fn view<T>(_nannou_app: &nannou::App, app: &App<T>, frame: Frame) {
    if let Ok(graphics) = app.graphics.try_borrow_mut() {
        three_d_view_rendering(graphics, &frame, &app.camera);
    }

    app.egui_instance
        .draw_to_frame(&frame)
        .expect("egui instance couldn't be drawn")
}
