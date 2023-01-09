use std::sync::Mutex;
use wgpu::{TextureUsages, util::DeviceExt};

pub struct OutputTexture {
    pub texture: wgpu::Texture,
    pub size: wgpu::Extent3d,
    pub view: wgpu::TextureView,
    pub raster_bind_group: wgpu::BindGroup,
    pub copy_bind_group: wgpu::BindGroup,
    pub clear_data: Mutex<Vec<u8>>,
    pub depth_buffer: wgpu::Buffer,
    pub depth_buffer_clear_data: Vec<f32>
}

impl OutputTexture {
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        raster_pipeline: &wgpu::ComputePipeline,
        copy_pipeline: &wgpu::RenderPipeline
    ) -> Self {
        let depth_buffer_clear_data = vec![f32::MAX;(width * height)as usize];
        let depth_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&depth_buffer_clear_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        });

        let size = wgpu::Extent3d {
            width, height,
            depth_or_array_layers: 1
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let raster_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raster_pipeline.get_bind_group_layout(1),
            entries: &vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: depth_buffer.as_entire_binding()
                }
            ]
        });
        let copy_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });
        let copy_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &copy_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&copy_sampler)
                }
            ]
        });
        let s = size.width * size.height * 4;
        let mut clear_data = Vec::with_capacity(s as usize);
        for _ in 0..s {
            clear_data.push(0);
        }
        Self {
            texture,
            size,
            view,
            raster_bind_group,
            copy_bind_group,
            clear_data: Mutex::new(clear_data),
            depth_buffer,
            depth_buffer_clear_data
        }
    }
}