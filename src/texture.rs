use std::num::NonZeroU32;
use crate::wgpu;
use crate::nannou::image;
use crate::error::RendError;

use nannou::image::GenericImageView;

pub(crate) struct Texture {
    pub _texture: wgpu::TextureHandle,
    pub view: wgpu::TextureViewHandle,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(
        device: &wgpu::Device,
        label: Option<&str>,
    ) -> Texture {
        let (x, y) = (4, 4);

        let size = wgpu::Extent3d {
            width: x,
            height: y,
            depth_or_array_layers: 1,
        };
        let texture : wgpu::TextureHandle = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let view : wgpu::TextureViewHandle = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            _texture: texture,
            view,
            sampler,
        }
    }

    pub fn from_file(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
    ) -> Result<Texture, Box<dyn std::error::Error>> {
        return match std::fs::read(path) {
            Ok(contents) => { Texture::from_bytes(device, queue, contents.as_slice(), label)},
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Texture, Box<dyn std::error::Error>> {
        if let Ok(img) = image::load_from_memory(bytes) {
            Texture::from_image(device, queue, &img, Some(label))
        } else {
            Err(Box::new(RendError::new("texture couldn't be loaded")))
        }
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Texture, Box<dyn std::error::Error>> {
        let rgba = img.to_rgba8();
        let (x, y) = img.dimensions();

        let size = wgpu::Extent3d {
            width: x,
            height: y,
            depth_or_array_layers: 1,
        };

        let mip_level_count = 1;
        let texture : wgpu::TextureHandle = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        if let (Some(nz_x), Some(nz_y)) = (NonZeroU32::new(x * 4), NonZeroU32::new(y)) {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(nz_x),
                    rows_per_image: Some(nz_y),
                },
                size,
            );
        }

        let view : wgpu::TextureViewHandle = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(Self {
            _texture: texture,
            view,
            sampler,
        })
    }
}