use glam::Vec3A;
use nannou::wgpu;
use wgpu::Device;
use wgpu::util::DeviceExt;

use crate::obj::{Indices, Mesh, Normals, Vertices};

pub(crate) fn vertices_as_bytes_copy(data: &Vec<glam::Vec3A>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        for i in 0..3 {
            final_bytes.extend(elem[i].to_le_bytes());
        }
    }
    final_bytes
}

pub(crate) fn indices_as_bytes_copy(data: &Vec<u16>) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = vec![];
    for elem in data {
        final_bytes.push(*elem as u8);
        final_bytes.push((*elem >> 8) as u8);
    }
    final_bytes
}

pub(crate) struct Buffers {
    indices: Indices,
    vertices: Vertices,
    uvs: Vertices,
    normals: Normals,
}

impl Buffers {
    pub(crate) fn len(&self) -> usize {
        self.indices.len()
    }

    pub(crate) fn new() -> Buffers {
        Buffers {
            indices: vec![],
            vertices: vec![],
            uvs: vec![],
            normals: vec![],
        }
    }
    
    pub(crate) fn load_mesh(&mut self, mesh: &mut Mesh) {
        let buffers = mesh.as_buffers();

        self.indices.extend(buffers.0);
        self.vertices.extend(buffers.1);
        self.uvs.extend(buffers.2);
        self.normals.extend(buffers.3);
    }

    pub(crate) fn as_wgpu_buffers(&self, device: &Device) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        let indices_bytes = indices_as_bytes_copy(&self.indices);
        let vertices_bytes = vertices_as_bytes_copy(&self.vertices);
        let uvs_bytes = vertices_as_bytes_copy(&self.uvs);
        let normals_bytes = vertices_as_bytes_copy(&self.normals);

        let index_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*indices_bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let vertex_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*vertices_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let uv_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*uvs_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let normal_buffer = device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: &*normals_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        (index_buffer, vertex_buffer, uv_buffer, normal_buffer)
    }
}

