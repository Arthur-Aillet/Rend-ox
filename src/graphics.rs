use nannou::wgpu;
use crate::mesh::Mesh;
use crate::Vec3;
use crate::glam::Quat;
use crate::app::{indices_as_bytes_copy, vertices_as_bytes_copy};
use nannou::wgpu::util::DeviceExt;

pub struct Graphics {
    //pub device: &wgpu::Device,
    pub uniform_buffer: wgpu::Buffer,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    // pub(crate) render_pass: Option<wgpu::RenderPass>,
}

impl Graphics {
    pub fn new(
        //device: &wgpu::Device,
        uniform_buffer: wgpu::Buffer,
        depth_texture: wgpu::Texture,
        depth_texture_view: wgpu::TextureView,
        bind_group: wgpu::BindGroup,
        render_pipeline: wgpu::RenderPipeline
    ) -> Graphics {
        Graphics {
            //device,
            uniform_buffer,
            depth_texture,
            depth_texture_view,
            bind_group,
            render_pipeline,
            // render_pass : None,
        }
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

