use winit::window::Window;
use crate::uniform::create_uniform_buffer;

pub struct Graphics {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) uv_buffer: wgpu::Buffer,
    pub(crate) normal_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_texture_view: wgpu::TextureView,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

fn vertices_as_bytes_copy(data: &Vec<glam::Vec3A>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        for i in 0..3 {
            final_bytes.extend(elem[i].to_le_bytes());
        }
    }
    final_bytes
}

fn indices_as_bytes_copy(data: &Vec<u16>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        final_bytes.push(*elem as u8);
        final_bytes.push((*elem >> 8) as u8);
    }
    final_bytes
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
        .depth_format(depth_format)
        .sample_count(sample_count)
        .build(device)
}

impl Graphics {
    pub fn new(window : &Window) -> Graphics {
        let device = window.device();
        let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
        let fs_desc = wgpu::include_wgsl!("shaders/fs.wgsl");
        let vs_mod = device.create_shader_module(&vs_desc);
        let fs_mod = device.create_shader_module(&fs_desc);

        let indices_bytes = indices_as_bytes_copy(&buffers.0);
        let vertices_bytes = vertices_as_bytes_copy(&buffers.1);
        let uvs_bytes = vertices_as_bytes_copy(&buffers.2);
        let normals_bytes = vertices_as_bytes_copy(&buffers.3);

        let vertex_usage = wgpu::BufferUsages::VERTEX;
        let index_usage = wgpu::BufferUsages::INDEX;
        let vertex_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*vertices_bytes,
            usage: vertex_usage,
        });
        let uv_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*uvs_bytes,
            usage: vertex_usage,
        });
        let normal_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*normals_bytes,
            usage: vertex_usage,
        });
        let index_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*indices_bytes,
            usage: index_usage,
        });


        let depth_texture = wgpu::TextureBuilder::new()
            .size([window_size.x, window_size.y])
            .format(wgpu::TextureFormat::Depth32Float)
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .sample_count(msaa_samples)
            .build(device);

        let depth_texture_view = depth_texture.view().build();
        let uniform_buffer = create_uniform_buffer(window_size, camera.calc_view_matrix());

        let bind_group_layout = create_bind_group_layout(device);
        let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);
        let pipeline_layout = create_pipeline_layout(device, &bind_group_layout);
        let format = Frame::TEXTURE_FORMAT;
        let msaa_samples = window.msaa_samples();
        let render_pipeline = create_render_pipeline(
            device,
            &pipeline_layout,
            &vs_mod,
            &fs_mod,
            format,
            wgpu::TextureFormat::Depth32Float,
            msaa_samples,
        );
        Graphics {
            vertex_buffer,
            uv_buffer,
            normal_buffer,
            index_buffer,
            uniform_buffer,
            depth_texture,
            depth_texture_view,
            bind_group,
            render_pipeline,
        }
    }
}