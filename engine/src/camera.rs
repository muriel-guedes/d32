use std::sync::Mutex;

use cgmath::{Vector3, InnerSpace};
use wgpu::{util::DeviceExt, Queue};
use winit::{window::Window, dpi::PhysicalSize};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    pub position: [f32;4],
    pub centre: [f32;4],
    pub u: [f32;4],
    pub v: [f32;4],
}

#[derive(Clone)]
pub struct CameraValues {
    pub position: Vector3<f32>,
    pub lookat: Vector3<f32>,
    pub up: Vector3<f32>,
    pub length: f32,
    pub horizontal_size: f32,
    pub aspect_ratio: f32,

    pub alignment: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub centre: Vector3<f32>
}
impl CameraValues {
    pub fn update(&mut self) {
        self.alignment = (self.lookat - self.position).normalize();
        self.u = self.alignment.cross(self.up).normalize();
        self.v = self.u.cross(self.alignment).normalize();
        self.centre = self.position + (self.alignment * self.length);
        self.u *= self.horizontal_size;
        self.v *= self.horizontal_size / self.aspect_ratio;
    }
}
impl Into<CameraBinding> for CameraValues {
    fn into(self) -> CameraBinding {
        CameraBinding {
            position: self.position.extend(1.).into(),
            centre: self.centre.extend(1.).into(),
            u: self.u.extend(1.).into(),
            v: self.v.extend(1.).into()
        }
    }
}

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub values: Mutex<CameraValues>
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        window: &Window,
        raytrace_shader: &wgpu::RenderPipeline
    ) -> Self {
        let values = CameraValues {
            position: [0., -2., 0.].into(),
            lookat: [0.;3].into(),
            up: [0., 0., 1.].into(),
            length: 1.,
            horizontal_size: 1.,
            aspect_ratio: 1.,

            alignment: [0.;3].into(),
            centre: [0.;3].into(),
            u: [0.;3].into(),
            v: [0.;3].into()
        };
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[ CameraBinding::from(values.clone().into()) ]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raytrace_shader.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ]
        });
        Self {
            buffer,
            bind_group,
            values: Mutex::new(values)
        }
    }
    pub fn update(&self, queue: &Queue) {
        let mut values = self.values.lock().unwrap();
        values.update();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[ CameraBinding::from(values.clone().into()) ]))
    }
    pub fn resize(&self, new_size: PhysicalSize<u32>) {
    }
}