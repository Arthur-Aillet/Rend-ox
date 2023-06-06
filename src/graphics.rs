use winit::window::Window;
use crate::uniform::create_uniform_buffer;
use wgpu;
use wgpu::Device;

pub struct Graphics {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) uv_buffer: wgpu::Buffer,
    pub(crate) normal_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) uniform_buffer: wgpu::Buffer,
/*    pub(crate) depth_texture: wgpu::Texture,
    pub(crate) depth_texture_view: wgpu::TextureView,
    pub(crate) bind_group: wgpu::BindGroup,*/
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


// fn create_line_render_pipline () {}
// fn create_point_render_pipline () {}

pub(crate) fn create_3d_render_pipeline (
    device: &wgpu::Device,
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    final_image_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let primitive = wgpu::PrimitiveState {  // Describe how the vertex buffers must be interpreted
        topology: wgpu::PrimitiveState::TriangleList, //Switch with TriangleStrip when parser adapted
        strip_index_format: Some(wgpu::IndexFormat::Uint32),
        front_face: wgpu::FrontFace::Ccw, // Front face is counter clock wise
        cull_mode: None, // Some(wgpu::Face::Back) to discard back facing polygons
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill, // could be line or point be require other feature
        conservative: false,
    };

    let sample_state = wgpu::MultisampleState {
        count: sample_count,
        mask: !0,
        alpha_to_coverage_enabled: false,
    };

    let depth_descriptor = wgpu::DepthStencilState { // describe depth buffer formating
        format: depth_format,
        depth_write_enabled: false,
        depth_compare: wgpu::CompareFunction::Never,
        stencil: Default::default(),
        bias: Default::default(),
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("3D rendering pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "main",
            buffers: &[], // tells wgpu what type of vertices we want to pass to the vertex shader. already specified the vertices in the vertex shader itself
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: final_image_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive,
        depth_stencil: Some(depth_descriptor),
        multisample: sample_state,
        multiview: None, //  One for now
        })
    }

impl Graphics {
    pub fn new(device : &Device) -> Graphics {
        let sample_count = 4;
        /*
        let vs_desc = wgpu::include_wgsl!("shaders/vs.wgsl");
        let fs_desc = wgpu::include_wgsl!("shaders/fs.wgsl");


        let vs_mod = device.create_shader_module(&vs_desc);
        let fs_mod = device.create_shader_module(&fs_desc);
        */
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("fragment"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/fs.wgsl").into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("vertex"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/vs.wgsl").into()),
        });


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

        /*
        let depth_texture = wgpu::TextureBuilder::new()
            .size([window_size.x, window_size.y])
            .format(wgpu::TextureFormat::Depth32Float)
            .usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .sample_count(msaa_samples)
            .build(device);

        let depth_texture_view = depth_texture.view().build();
        let bind_group_layout = create_bind_group_layout(device);
        let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);
        let format = Frame::TEXTURE_FORMAT; */

        let uniform_buffer = create_uniform_buffer(window_size, camera.calc_view_matrix());

        let render_pipeline = create_3d_render_pipeline(
            device,
            &vertex_shader,
            &fragment_shader,
            wgpu::TextureFormat::Rgba32Sint,
            wgpu::TextureFormat::R8Unorm,
            sample_count);

        Graphics {
            vertex_buffer,
            uv_buffer,
            normal_buffer,
            index_buffer,
            uniform_buffer,
            render_pipeline,
        }
    }
}