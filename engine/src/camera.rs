use std::sync::Mutex;
use cgmath::{Matrix4, Quaternion, Rotation3, Vector3, Rad, Point3};
use wgpu::{util::DeviceExt, Queue};
use winit::{window::Window, dpi::PhysicalSize};

use crate::{settings::Settings, utils::{SmoothValue, SmoothValueBounded}, cursor::Cursor};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraBinding {
    projection: [[f32;4];4],
    position: [f32;4]
}

pub struct CameraValues {
    perspective: Matrix4<f32>,
    position: Vector3<f32>,
    rotation_x: SmoothValue<f32>,
    rotation_y: SmoothValue<f32>,
    distance: SmoothValueBounded<f32>
}
impl CameraValues {
    pub fn new(
        settings: &Settings,
        window: &Window
    ) -> Self {
        let window_size = window.inner_size();
        let aspect = window_size.width as f32 / window_size.height as f32;
        let perspective = cgmath::perspective(cgmath::Deg(settings.fov), aspect, settings.near, settings.far);
        Self {
            perspective,
            position: Vector3::new(0., 0., 0.),
            rotation_x: SmoothValue::new(0., 0.0015, 0.12),
            rotation_y: SmoothValue::new(0., 0.0015, 0.12),
            distance: SmoothValueBounded::new(8., 0.5, 0.1, 1., 64.)
        }
    }
    pub fn update(&mut self, cursor: &Cursor) {
        self.distance.change(-cursor.wheel_movement());
        let movement = cursor.get_movement();
        self.rotation_x.change(-movement.y as f32);
        self.rotation_y.change(movement.x as f32);
        let rotation = Quaternion::from_angle_y(Rad(self.rotation_y.get())) * Quaternion::from_angle_x(Rad(self.rotation_x.get()));
        self.position = rotation * Vector3::new(0., 0., self.distance.get());
    }
    pub fn get_projection(&self) -> Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            Point3::from_homogeneous(self.position.extend(1.)),
            [0.;3].into(),
            [0., 1., 0.].into()
        );
        (self.perspective * view).into()
    }
    pub fn resize(&mut self, settings: &Settings, new_size: PhysicalSize<u32>) {
        let aspect = new_size.width as f32 / new_size.height as f32;
        self.perspective = cgmath::perspective(cgmath::Deg(settings.fov), aspect, settings.near, settings.far);
    }
    pub fn get_position(&self) -> [f32;4] {
        [self.position.x, self.position.y, self.position.z, 1.]
    }
}

pub struct Camera {
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub values: Mutex<CameraValues>
}

impl Camera {
    pub fn new(
        settings: &Settings,
        device: &wgpu::Device,
        window: &Window,
        cursor: &Cursor,
        raster_pipeline: &wgpu::ComputePipeline
    ) -> Self {
        let mut values = CameraValues::new(settings, window);
        values.update(cursor);
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[CameraBinding {
                    projection: values.get_projection().into(),
                    position: values.get_position()
                }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raster_pipeline.get_bind_group_layout(2),
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
    pub fn update(&self, queue: &Queue, cursor: &Cursor) {
        let mut values = self.values.lock().unwrap();
        values.update(cursor);
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[CameraBinding {
            projection: values.get_projection().into(),
            position: values.get_position()
        }]))
    }
    pub fn resize(&self, settings: &Settings, new_size: PhysicalSize<u32>) {
        self.values.lock().unwrap().resize(settings, new_size)
    }
}