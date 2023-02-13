use std::sync::Mutex;
use wgpu::util::DeviceExt;

use crate::Chunk;

pub struct Chunks {
    pub current_length: usize,
    pub maximum_length: usize,
    pub length_buffer: wgpu::Buffer,

    pub chunks: Vec<Chunk>,
    pub chunks_buffer: Mutex<wgpu::Buffer>,

    pub bind_group: Mutex<wgpu::BindGroup>,
}
impl Chunks {
    pub fn new(
        device: &wgpu::Device,
        raytrace_shader: &wgpu::RenderPipeline
    ) -> Self {
        let length_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[0u32;4]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let chunks_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vec![
                    Chunk::new()
                ]),
                usage: wgpu::BufferUsages::STORAGE
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raytrace_shader.get_bind_group_layout(1),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: chunks_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: length_buffer.as_entire_binding()
                }
            ]
        });
        Self {
            current_length: 0,
            maximum_length: 0,
            length_buffer,

            chunks_buffer: Mutex::new(chunks_buffer),
            chunks: vec![],
            
            bind_group: Mutex::new(bind_group)
        }
    }
    pub fn update(&self) {
    }
}