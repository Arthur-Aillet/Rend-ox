use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;

use crate::mesh::Mesh;
use crate::Vec3;
use crate::glam::Quat;
use crate::app::{indices_as_bytes_copy, vertices_as_bytes_copy};
use crate::mesh::MeshDescriptor;
use crate::error::RendError;
use crate::uniforms::Uniforms;
use crate::Mat4;

use nannou::wgpu;
use nannou::wgpu::util::DeviceExt;
use nannou::Frame;
use nannou::text::font::default;
use nannou::wgpu::ShaderModule;
use crate::camera::Camera;

pub struct Graphics {
    // pub device: &'static wgpu::Device,
    pub uniform_buffer: wgpu::Buffer,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub render_pipelines: HashMap<usize, wgpu::RenderPipeline>,
    pipeline_layout: wgpu::PipelineLayout,
    pub(crate) draw_queue: HashMap<MeshDescriptor, Vec<Mat4>>,
    pub shaders: HashMap<usize, wgpu::ShaderModule>,
    pub shader_sources: HashMap<usize, wgpu::ShaderModuleDescriptor<'static>>,
    pub meshes: HashMap<usize, Mesh>,
    mesh_count: usize,
    default_shader: usize,
    vs_mod: wgpu::ShaderModule,
    msaa: u32,
    // pub(crate) render_pass: Option<wgpu::RenderPass>,
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

impl Graphics {
    pub fn new(
        //device: &wgpu::Device,
        uniform_buffer: wgpu::Buffer,
        depth_texture: wgpu::Texture,
        depth_texture_view: wgpu::TextureView,
        bind_group: wgpu::BindGroup,
        pipeline_layout: wgpu::PipelineLayout,
        // render_pipelines: HashMap<usize, wgpu::RenderPipeline>,
        default_shader: usize,
        vs_mod: wgpu::ShaderModule,
        msaa: u32,
    ) -> Graphics {
        Graphics {
            //device,
            uniform_buffer,
            depth_texture,
            depth_texture_view,
            bind_group,
            render_pipelines: HashMap::new(),
            pipeline_layout,
            draw_queue: HashMap::new(),
            shaders: HashMap::new(),
            shader_sources: HashMap::new(),
            meshes: HashMap::new(),
            mesh_count: 0,
            default_shader,
            vs_mod,
            msaa,
            // render_pass : None,
        }
    }

    pub fn create(window: &nannou::window::Window, camera: &Camera) -> Graphics {
        let device = window.device();

        let format = Frame::TEXTURE_FORMAT;
        let msaa_samples = window.msaa_samples();
        let window_size: glam::UVec2 = window.inner_size_pixels().into();

        let vs_mod = device.create_shader_module(&wgpu::include_wgsl!("./shaders/vs.wgsl"));
        // let fs_mod = device.create_shader_module(&wgpu::include_wgsl!("./shaders/fs.wgsl"));

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
        // let mut render_pipelines = HashMap::new();
        // render_pipelines.insert() create_render_pipeline(
        //     device,
        //     &pipeline_layout,
        //     &vs_mod,
        //     &fs_mod,
        //     format,
        //     wgpu::TextureFormat::Depth32Float,
        //     msaa_samples,
        // )];

        let mut graphics = Graphics::new(
            // index_count,
            // index_buffer,
            // vertex_buffer,
            // uv_buffer,
            // normal_buffer,
            //device,
            uniform_buffer,
            depth_texture,
            depth_texture_view,
            bind_group,
            // render_pipelines,
            pipeline_layout,
            0,
            vs_mod,
            msaa_samples,
        );
        if let Ok(default_shader) = graphics.load_shader("./src/rend_ox/src/shaders/fs.wgsl") {
            graphics.default_shader = default_shader;
            graphics.refresh_shaders(device);
            println!("loaded fs as {}", default_shader);
        } else {
            println!("rend-ox: warning: default shader failed to load");
        }
        graphics
    }

    pub fn load_shader(&mut self, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
        return match std::fs::read_to_string(path) {
            Ok(shader_source) => {
                let idx = self.shader_sources.len();
                self.shader_sources.insert(idx,
                                           wgpu::ShaderModuleDescriptor {
                                               label: None,
                                               source: wgpu::ShaderSource::Wgsl(shader_source.into()),
                                           });
                println!("LOAD loaded {} as {}", path, idx);
                Ok(idx)
            }
            Err(e) => { println!("LOAD FAILED: {}", e); Err(Box::new(e))}
        }

    }

    pub fn refresh_shaders(&mut self, device: &wgpu::Device) {
        if self.shader_sources.len() > self.shaders.len() {
            for (idx, source) in &self.shader_sources {
                if !self.shaders.contains_key(&idx) {
                    let fs_mod = device.create_shader_module(&source);
                    self.render_pipelines.insert(*idx, self.create_render_pipeline_for_shader(device, &fs_mod));
                    self.shaders.insert(*idx, fs_mod);
                }
            }
        }
    }

    pub fn bind_shader_to_mesh(&self, md: &mut MeshDescriptor, shader : &usize) -> bool {
        if self.shader_sources.contains_key(shader) {
            md.shader = *shader;
            return true;
        }
        false
    }

    pub fn load_mesh(&mut self, path: &str) -> Result<MeshDescriptor, Box<dyn std::error::Error>> {
        for (idx, mesh) in &self.meshes {
            if mesh.path == path {
                return Ok(MeshDescriptor::new(*idx, path, self.default_shader));
            }
        }
        match Mesh::from_obj(path) {
            Ok(mesh) => { self.meshes.insert(self.mesh_count, mesh); }
            Err(e) => { return Err(e) }
        }
        let ret = MeshDescriptor::new(self.mesh_count, path, self.default_shader);
        self.mesh_count += 1;
        Ok(ret)
    }

    fn create_render_pipeline_for_shader(&self, device: &wgpu::Device, shader: &wgpu::ShaderModule) -> wgpu::RenderPipeline {
        create_render_pipeline(
            device,
            &self.pipeline_layout,
            &self.vs_mod,
            shader,
            wgpu::RenderPipelineBuilder::DEFAULT_COLOR_FORMAT,
            wgpu::RenderPipelineBuilder::DEFAULT_DEPTH_FORMAT,
            self.msaa,
        )
    }


    pub fn draw(&self, device: &wgpu::Device, buffers: &mut Vec<wgpu::Buffer>, counts: &mut Vec<usize>, mesh : &Mesh) -> bool {
        let indices_bytes = indices_as_bytes_copy(&mesh.faces);
        let vertices_bytes = vertices_as_bytes_copy(&mesh.vertices);
        let uvs_bytes = vertices_as_bytes_copy(&mesh.uvs);
        let normals_bytes = vertices_as_bytes_copy(&mesh.normals);

        let index_count = mesh.faces.len();

        counts.push(index_count);
        buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*indices_bytes,
            usage: wgpu::BufferUsages::INDEX,
        }));
        buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*vertices_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        }));
        buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*uvs_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        }));
        buffers.push(device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*normals_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        }));


         return true;
    }

    // pub fn draw_instanced(&mut self, mesh : Mesh, instances: Vec<(Vec3, Quat)>) -> bool {
        // if let Some(render_pass) = self.render_pass {
        //     let index_count = mesh.faces.len();
        //     let indices_bytes = indices_as_bytes_copy(&mesh.faces);
        //     let vertices_bytes = vertices_as_bytes_copy(&mesh.vertices);
        //     let uvs_bytes = vertices_as_bytes_copy(&mesh.uvs);
        //     let normals_bytes = vertices_as_bytes_copy(&mesh.normals);
        //
        //     let index_buffer = self.device.create_buffer_init(&wgpu::BufferInitDescriptor {
        //         label: None,
        //         contents: &*indices_bytes,
        //         usage: wgpu::BufferUsages::INDEX,
        //     });
        //     let vertex_buffer = self.device.create_buffer_init(&wgpu::BufferInitDescriptor {
        //         label: None,
        //         contents: &*vertices_bytes,
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        //     let uv_buffer = self.device.create_buffer_init(&wgpu::BufferInitDescriptor {
        //         label: None,
        //         contents: &*uvs_bytes,
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        //     let normal_buffer = self.device.create_buffer_init(&wgpu::BufferInitDescriptor {
        //         label: None,
        //         contents: &*normals_bytes,
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        //
        //     render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        //     render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        //     render_pass.set_vertex_buffer(1, uv_buffer.slice(..));
        //     render_pass.set_vertex_buffer(2, normal_buffer.slice(..));
        //     render_pass.draw_indexed(0..index_count as u32, 0, 0..1);
        //     true
        // } else {
        //     false
        // }
    //     false
    // }
}
