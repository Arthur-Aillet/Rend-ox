use std::collections::HashMap;
use std::mem::{size_of, swap};

use crate::app::{indices_as_bytes_copy, vertices_as_bytes_copy};
use crate::mesh::Mesh;
use crate::mesh::MeshDescriptor;
use crate::uniforms::Uniforms;
use crate::Mat4;
use crate::Vec3;

use crate::camera::Camera;
use nannou::wgpu;
use nannou::wgpu::util::DeviceExt;
use crate::material::{Material, MaterialDescriptor};

pub type ShaderSlot = usize;
pub type MaterialSlot = usize;
pub type MeshSlot = usize;

pub struct Graphics {
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
    pub(crate) meshes:      HashMap<MeshSlot    , Mesh>,
    pub(crate) materials:   HashMap<MaterialSlot, Material>,
    pub material_layout:    Option<wgpu::BindGroupLayout>,
    pub material_sources:   HashMap<MaterialSlot, MaterialDescriptor>,
    pub(crate) shaders:     HashMap<ShaderSlot  , wgpu::ShaderModule>,
    pub shader_sources:     HashMap<ShaderSlot  , wgpu::ShaderModuleDescriptor<'static>>,
    pub render_pipelines:   HashMap<ShaderSlot  , wgpu::RenderPipeline>,
    pipeline_layout: wgpu::PipelineLayout,
    pub(crate) draw_queue:  HashMap<MeshDescriptor  , (Vec<Vec3>, Vec<Mat4>)>,
    default_material: ShaderSlot,
    vs_mod: wgpu::ShaderModule,
    msaa: u32,
    // pub(crate) render_pass: Option<wgpu::RenderPass>,
}

fn create_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
        .build(device)
}

fn create_uniform_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .buffer::<Uniforms>(uniform_buffer, 0..1)
        .build(device, layout)
}

fn create_material_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
        .texture(
            wgpu::ShaderStages::FRAGMENT,
            false,
            wgpu::TextureViewDimension::D2,
            wgpu::TextureSampleType::Float { filterable: true })
        .sampler(
            wgpu::ShaderStages::FRAGMENT,
            true
        )
        .build(device)
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    mat_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&uniform_bind_group_layout, &mat_bind_group_layout],
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
        .add_instance_buffer::<glam::Vec3>(&wgpu::vertex_attr_array![9 => Float32x3])
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
            },
        ])
        .depth_format(depth_format)
        .sample_count(sample_count)
        .build(device)
}

impl Graphics {
    pub fn new(
        uniform_buffer: wgpu::Buffer,
        uniform_bind_group: wgpu::BindGroup,
        depth_texture: wgpu::Texture,
        depth_texture_view: wgpu::TextureView,
        pipeline_layout: wgpu::PipelineLayout,
        material_layout: Option<wgpu::BindGroupLayout>,
        default_material: ShaderSlot,
        vs_mod: wgpu::ShaderModule,
        msaa: u32,
    ) -> Graphics {
        Graphics {
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            depth_texture_view,
            meshes: HashMap::new(),
            materials: HashMap::new(),
            material_layout,
            material_sources: HashMap::new(),
            shaders: HashMap::new(),
            shader_sources: HashMap::new(),
            render_pipelines: HashMap::new(),
            pipeline_layout,
            draw_queue: HashMap::new(),
            // material,
            default_material,
            vs_mod,
            msaa,
            // render_pass : None,
        }
    }

    pub fn create(window: &nannou::window::Window, camera: &Camera) -> Graphics {
        let device = window.device();
        let queue = window.queue();

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
        let uniform_bind_group_layout = create_uniform_bind_group_layout(device);
        let uniform_bind_group = create_uniform_bind_group(device, &uniform_bind_group_layout, &uniform_buffer);

        let material_bind_group_layout = create_material_bind_group_layout(device);
        // let material_bind_group = create_material_bind_group(device, &material_bind_group_layout, &material_buffer);

        let pipeline_layout = create_pipeline_layout(device, &uniform_bind_group_layout, &material_bind_group_layout);

        // let texture = Texture::from_file(device, queue,"happy-tree.png", "tree").expect("failed to load default tex");
        //
        // let material_bind_group_layout = create_material_bind_group_layout(device);
        // let material_bind_group = create_material_bind_group(device, &material_bind_group_layout, &diffuse_texture_view, diffuse_sampler);

        let mut graphics = Graphics::new(
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
            depth_texture_view,
            pipeline_layout,
            Some(material_bind_group_layout),
            0,
            vs_mod,
            msaa_samples,
        );

        let mut mat = MaterialDescriptor::new();
        mat.shader = Some("./src/rend_ox/src/shaders/fs.wgsl".into());

        let default_material = graphics.load_material(mat);
        graphics.default_material = default_material;
        graphics.refresh_ressources(device, queue);
        println!("loaded fs as {}", default_material);
        graphics
    }

    pub fn load_material(&mut self, material: MaterialDescriptor) -> MaterialSlot {
        let idx = self.material_sources.len() as MaterialSlot;
        self.material_sources.insert(idx, material);
        idx
    }

    pub fn load_shader(&mut self, path: &str) -> Result<ShaderSlot, Box<dyn std::error::Error>> {
        return match std::fs::read_to_string(path) {
            Ok(shader_source) => {
                let idx = self.shader_sources.len() as ShaderSlot;
                self.shader_sources.insert(
                    idx,
                    wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
                    },
                );
                println!("LOAD loaded {} as {}", path, idx);
                Ok(idx)
            }
            Err(e) => {
                println!("LOAD FAILED: {}", e);
                Err(Box::new(e))
            }
        };
    }

    pub fn refresh_ressources(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.material_sources.len() > self.materials.len() {
            let material_sources= std::mem::take(&mut self.material_sources);
            if let Some(material_layout) = std::mem::take(&mut self.material_layout) {
                for (idx, source) in &material_sources {
                    if !self.materials.contains_key(idx) {
                        let mat = Material::from_descriptor(self, device, queue, &material_layout, &source);
                        self.materials.insert(*idx, mat);
                    }
                }
                std::mem::replace(&mut self.material_layout, Some(material_layout));
            }
            std::mem::replace(&mut self.material_sources, material_sources);
        }
        if self.shader_sources.len() > self.shaders.len() {
            for (idx, source) in &self.shader_sources {
                if !self.shaders.contains_key(&idx) {
                    let fs_mod = device.create_shader_module(&source);
                    self.render_pipelines.insert(
                        *idx,
                        self.create_render_pipeline_for_shader(device, &fs_mod),
                    );
                    self.shaders.insert(*idx, fs_mod);
                }
            }
        }
    }

    pub fn bind_material_to_mesh(&self, md: &mut MeshDescriptor, material: &MaterialSlot) -> bool {
        if self.material_sources.contains_key(material) {
            md.material = *material;
            return true;
        }
        false
    }

    pub fn load_mesh(&mut self, path: &str) -> Result<MeshDescriptor, Box<dyn std::error::Error>> {
        for (idx, mesh) in &self.meshes {
            if mesh.path == path {
                return Ok(MeshDescriptor::new(*idx, path, self.default_material));
            }
        }
        return match Mesh::from_obj(path) {
            Ok(mesh) => {
                let idx = self.meshes.len() as MeshSlot;
                self.meshes.insert(idx, mesh);
                Ok(MeshDescriptor::new(idx, path, self.default_material))
            }
            Err(e) => Err(e)
        };
    }

    fn create_render_pipeline_for_shader(
        &self,
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
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

    pub fn draw(
        &self,
        device: &wgpu::Device,
        buffers: &mut Vec<wgpu::Buffer>,
        counts: &mut Vec<usize>,
        mesh: &Mesh,
    ) -> bool {
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
