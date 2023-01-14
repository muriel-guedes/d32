use std::sync::{Mutex, atomic::AtomicBool, Arc};

use cgmath::Vector3;
use wgpu::util::DeviceExt;

use crate::{Color, COLOR, Context};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkBinding {
    pub position: [f32;4],
    pub size: [u32;4]
}

#[derive(Clone)]
pub struct Chunk {
    pub data: Arc<Mutex<Vec<COLOR>>>,
    pub buffer: Arc<wgpu::Buffer>,
    pub bind_group: Arc<wgpu::BindGroup>,
    pub needs_update: Arc<AtomicBool>,
    pub position: Arc<Mutex<Vector3<f32>>>,
    pub size: Arc<Vector3<u32>>
}
impl Chunk {
    pub fn new(
        device: &wgpu::Device,
        raster_pipeline: &wgpu::ComputePipeline,
        data: Vec<COLOR>,
        position: Vector3<f32>,
        size: Vector3<u32>
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        });
        let binding = ChunkBinding {
            position: position.extend(1.).into(),
            size: [size.x, size.y, size.z, size.x * size.y].into()
        };
        let chunk_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[binding]),
            usage: wgpu::BufferUsages::STORAGE
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raster_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: chunk_buffer.as_entire_binding()
                }
            ]
        });
        Self {
            data: Arc::new(Mutex::new(data)),
            buffer: Arc::new(buffer),
            bind_group: Arc::new(bind_group),
            needs_update: Arc::new(AtomicBool::new(true)),
            position: Arc::new(Mutex::new(position)),
            size: Arc::new(size)
        }
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        if !self.needs_update.load(std::sync::atomic::Ordering::Relaxed) { return }
        self.needs_update.store(false, std::sync::atomic::Ordering::Relaxed);
        let data = self.data.lock().unwrap();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }
    pub fn new_gradient_cube(
        c: &Context,
        position: Vector3<f32>,
        size: Vector3<u32>
    ) -> Self {
        let data_length = size.x + (size.y * size.x) + (size.z * size.y * size.x);
        let mut data = Vec::with_capacity(data_length as usize);
        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    if x == size.x - 1 || x == 0 || y == size.y - 1 || y == 0 || z == size.z - 1 || z == 0 {
                        data.push(Color::from((
                            ((x as f32 / size.x as f32) * 255.) as u8,
                            ((y as f32 / size.y as f32) * 255.) as u8,
                            ((z as f32 / size.z as f32) * 255.) as u8
                        )).get())
                    } else {
                        data.push(0)
                    }
                }
            }
        }
        Self::new(&c.device, &c.raster_shader.pipeline, data, position, size)
    }
    pub fn new_gradient_filled_cube(
        c: &Context,
        position: Vector3<f32>,
        size: Vector3<u32>
    ) -> Self {
        let data_length = size.x + (size.y * size.x) + (size.z * size.y * size.x);
        let mut data = Vec::with_capacity(data_length as usize);
        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    data.push(Color::from((
                        ((x as f32 / size.x as f32) * 255.) as u8,
                        ((y as f32 / size.y as f32) * 255.) as u8,
                        ((z as f32 / size.z as f32) * 255.) as u8
                    )).get())
                }
            }
        }
        Self::new(&c.device, &c.raster_shader.pipeline, data, position, size)
    }
}