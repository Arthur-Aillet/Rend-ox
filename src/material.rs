use crate::texture::Texture;
use crate::graphics::{Graphics, ShaderSlot};
use crate::wgpu;
use crate::glam::Vec4;
use nannou::prelude::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MaterialData {
    pub(crate) color: Vec4,
    pub(crate) specular: Vec4,
    pub(crate) field3: Vec4,
    pub(crate) field4: Vec4,
}

impl MaterialData {
    fn as_bytes_copy(&self) -> Vec<u8> {
        let mut final_bytes: Vec<u8> = vec![];
        for vec in [self.color, self.specular, self.field3, self.field4].iter() {
            for i in 0..4 {
                final_bytes.extend(vec.to_array()[i].to_le_bytes());
            }
        }
        final_bytes
    }

    pub(crate) fn as_buffer(&self, device: &nannou::wgpu::Device) -> wgpu::Buffer {
        let bytes = self.as_bytes_copy();
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;

        device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: Some("Uniform buffer"),
            contents: &*bytes,
            usage,
        })
    }
}

#[derive(Clone, Debug)]
pub struct MaterialDescriptor {
    pub data: MaterialData,
    pub maps: Vec<String>,
    pub shader: Option<String>,
}

impl MaterialDescriptor {
    pub fn new() -> Self {
        Self {
            data: MaterialData::new(),
            maps: vec![],
            shader: None,
        }
    }
}

pub(crate) struct Material {
    pub(crate) _data: MaterialData,
    pub(crate) _buffer: wgpu::Buffer,
    pub(crate) _maps: Vec<Texture>,
    pub(crate) group: wgpu::BindGroup,
    pub shader: ShaderSlot,
}

impl MaterialData {
    pub fn new() -> Self {
        Self {
            color: Vec4::new(1., 1., 1., 1.),
            specular: Vec4::new(1.0, 1.0, 1.0, 0.5),
            field3: Vec4::new(0., 0., 0., 0.),
            field4: Vec4::new(0., 0., 0., 0.),
        }
    }

    pub fn diffuse(color: Vec4) -> Self {
        Self {
            color,
            specular: Vec4::new(1.0, 1.0, 1.0, 0.5),
            field3: Vec4::new(0., 0., 0., 0.),
            field4: Vec4::new(0., 0., 0., 0.),
        }
    }
}

impl Material {
    pub fn from_descriptor(g: &mut Graphics, device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout, mat: &MaterialDescriptor) -> Self {
        let buffer = mat.data.as_buffer(device);
        let mut maps = vec![];
        for i in 0..1 {
            let path: &str;
            if let Some(p) = mat.maps.get(i) {
                path = p.as_str();
            } else {
                path = match i {
                    0 => "dev/white.png",
                    1 => "dev/nm.png",
                    _ => "dev/white.png",
                }
            }
            if let Ok(tex) = Texture::from_file(device, queue, path, "dynamic") {
                maps.push(tex);
            } else {
                maps.push(Texture::new(device, Some("black")));
            }
        }
        let group = Self::bind_group(&maps, &buffer, device, layout);
        let shader = match &mat.shader {
            Some(path) => g.load_shader(path.as_str()).unwrap_or(ShaderSlot::default()),
            None => ShaderSlot::default(),
        };
        Self {
            _data: mat.data,
            _buffer: buffer,
            _maps: maps,
            shader,
            group,
        }
    }

    fn bind_group(maps: &Vec<Texture>, buffer: &wgpu::Buffer, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        println!("making bindgroup for {} maps", maps.len());
        let mut group = wgpu::BindGroupBuilder::new()
            .buffer::<MaterialData>(buffer, 0..1);
        let map = maps.get(0).expect("NO FILE GIVEN");
        group = group.texture_view(&map.view)
            .sampler(&map.sampler);
        group.build(device, layout)
    }
}