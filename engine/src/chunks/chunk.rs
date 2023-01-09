use std::sync::{Mutex, atomic::AtomicBool};

use wgpu::util::DeviceExt;

use crate::Color;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_LENGTH: usize = 16;
pub const CHUNK_SIZE: usize = CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_LENGTH;

pub struct Chunk {
    pub data: Mutex<[u32;CHUNK_SIZE]>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub needs_update: AtomicBool
}
impl Chunk {
    pub fn new(
        device: &wgpu::Device,
        raster_pipeline: &wgpu::ComputePipeline
    ) -> Self {
        let mut data = [0;CHUNK_SIZE];
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_LENGTH {
                    let i = x + (y*CHUNK_WIDTH) + (z*CHUNK_WIDTH*CHUNK_HEIGHT);
                    data[i] = Color::from((
                        ((x as f32 / CHUNK_WIDTH as f32) * 255.) as u8,
                        ((y as f32 / CHUNK_HEIGHT as f32) * 255.) as u8,
                        ((z as f32 / CHUNK_LENGTH as f32) * 255.) as u8
                    )).to_u32();
                }
            }
        }
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raster_pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding()
            }]
        });
        Self {
            data: Mutex::new(data),
            buffer,
            bind_group,
            needs_update: AtomicBool::new(true)
        }
    }
    pub fn set_pixel(&self, x: usize, mut y: usize, mut z: usize, color: impl Into<Color>) {
        let mut data = self.data.lock().unwrap();
        y *= CHUNK_WIDTH;
        z *= CHUNK_WIDTH * CHUNK_HEIGHT;
        data[x + y + z] = color.into().to_u32();
        self.needs_update.store(true, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        if !self.needs_update.load(std::sync::atomic::Ordering::Relaxed) { return }
        self.needs_update.store(false, std::sync::atomic::Ordering::Relaxed);
        let data = *self.data.lock().unwrap();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[data]));
    }
}