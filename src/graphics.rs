use nannou::wgpu;

pub struct Graphics {
    pub vertex_buffer: wgpu::Buffer,
    pub uv_buffer: wgpu::Buffer,
    pub normal_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub uniform_buffer: wgpu::Buffer,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Graphics {
    pub fn new(
        vertex_buffer: wgpu::Buffer,
        uv_buffer: wgpu::Buffer,
        normal_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        uniform_buffer: wgpu::Buffer,
        depth_texture: wgpu::Texture,
        depth_texture_view: wgpu::TextureView,
        bind_group: wgpu::BindGroup,
        render_pipeline: wgpu::RenderPipeline
    ) -> Graphics {
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