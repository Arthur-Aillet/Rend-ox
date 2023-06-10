use std::cell::RefCell;
use std::mem::size_of;

use crate::Vec3;
use nannou;
use nannou::wgpu;
use nannou::Frame;
use nannou_egui::Egui;
use glam::Mat4;

use crate::camera_controller::key_pressed;
use crate::graphics::Graphics;
use crate::mesh::Mesh;
use crate::process::{view, event, update};
use crate::uniforms::Uniforms;

pub struct App<T> {
    pub camera_is_active: bool,
    pub graphics: RefCell<Graphics>,
    pub camera: crate::camera::Camera,
    pub mesh: Mesh,
    pub egui_instance: Egui,
    pub user: T,
    pub user_update: UpdateFn<T>
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStages::VERTEX, false)
        .build(device)
}

fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .buffer::<Uniforms>(uniform_buffer, 0..1)
        .build(device, layout)
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    };
    device.create_pipeline_layout(&desc)
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(dst_format)
        .color_blend(wgpu::BlendComponent::REPLACE)
        .alpha_blend(wgpu::BlendComponent::REPLACE)
        .add_vertex_buffer::<glam::Vec3>(&wgpu::vertex_attr_array![0 => Float32x3])
        .add_vertex_buffer::<glam::Vec3>(&wgpu::vertex_attr_array![1 => Float32x3])
        .add_vertex_buffer::<glam::Vec3>(&wgpu::vertex_attr_array![2 => Float32x3])
        // instance matrix split into 4 vec4
        .add_instance_buffer::<glam::Mat4>(&[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 5,
                format: wgpu::VertexFormat::Float32x4,
            },
            // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
            // for each vec4. We'll have to reassemble the mat4 in
            // the shader.
            wgpu::VertexAttribute {
                offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 6,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                shader_location: 7,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: size_of::<[f32; 12]>() as wgpu::BufferAddress,
                shader_location: 8,
                format: wgpu::VertexFormat::Float32x4,
            }
        ])
        .depth_format(depth_format)
        .sample_count(sample_count)
        .build(device)
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
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let msaa_samples = window.msaa_samples();
    let window_size: glam::UVec2 = window.inner_size_pixels().into();

    let vs_mod = device.create_shader_module(&wgpu::include_wgsl!("./shaders/vs.wgsl"));
    let fs_mod = device.create_shader_module(&wgpu::include_wgsl!("./shaders/fs.wgsl"));

    let camera = crate::camera::Camera::new();

    let depth_texture = wgpu::TextureBuilder::new()
        .size([window_size.x, window_size.y])
        .format(wgpu::TextureFormat::Depth32Float)
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
        .sample_count(msaa_samples)
        .build(device);

    let depth_texture_view = depth_texture.view().build();

    let uniform_buffer = Uniforms::new_as_buffer(window_size, &camera, device);
    let bind_group_layout = create_bind_group_layout(device);
    let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);
    let pipeline_layout = create_pipeline_layout(device, &bind_group_layout);
    let render_pipeline = create_render_pipeline(
        device,
        &pipeline_layout,
        &vs_mod,
        &fs_mod,
        format,
        wgpu::TextureFormat::Depth32Float,
        msaa_samples,
    );

    let graphics = RefCell::new(Graphics::new(
        // index_count,
        // index_buffer,
        // vertex_buffer,
        // uv_buffer,
        // normal_buffer,
        // device,
        uniform_buffer,
        depth_texture,
        depth_texture_view,
        bind_group,
        render_pipeline,
    ));

    println!("Use the `W`, `A`, `S`, `D`, `Q` and `E` keys to move the camera.");
    println!("Use the mouse to orient the pitch and yaw of the camera.");
    println!("Press the `Space` key to toggle camera mode.");

    let ret = Mesh::from_obj("./.objs/bat.obj");
    match ret {
        Err(e) => return Err(e),
        Ok(mesh) => {
            Ok(App {
                camera_is_active,
                graphics,
                camera,
                mesh,
                user,
                user_update,
                egui_instance,
            })
        }
    }
}
