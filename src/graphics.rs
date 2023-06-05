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

impl Graphics {
    pub fn new() -> Graphics {
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