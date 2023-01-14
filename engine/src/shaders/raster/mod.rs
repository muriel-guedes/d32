use std::{borrow::Cow, sync::Mutex};
use winit::{window::Window, dpi::PhysicalSize};

use crate::Context;

mod output_texture;  pub use output_texture::*;

pub struct RasterShader {
    pub pipeline: wgpu::ComputePipeline,
    pub output: Mutex<Output>
}
impl RasterShader {
    pub fn new(
        device: &wgpu::Device,
        window: &Window,
        copy_pipeline: &wgpu::RenderPipeline
    ) -> Self {
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
            }),
            entry_point: "main"
        });
        let ws = window.inner_size();
        let output = Output::new(device, ws.width, ws.height, &pipeline, copy_pipeline);
        Self {
            pipeline,
            output: Mutex::new(output)
        }
    }
    pub fn draw(c: &Context) {
        let output_texture = c.raster_shader.output.lock().unwrap();
        c.queue.write_buffer(&output_texture.depth_buffer, 0, bytemuck::cast_slice(
            &output_texture.depth_buffer_clear_data
        ));
        c.queue.write_buffer(&output_texture.output_buffer, 0, bytemuck::cast_slice(
            &output_texture.output_buffer_clear_data
        ));

        let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let chunks = c.chunks.lock().unwrap();

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&c.raster_shader.pipeline);
            cpass.set_bind_group(1, &output_texture.raster_bind_group, &[]);
            cpass.set_bind_group(2, &c.camera.bind_group, &[]);
            for chunk in chunks.iter() {
                cpass.set_bind_group(0, &chunk.bind_group, &[]);
                cpass.dispatch_workgroups(chunk.size.x / 16, chunk.size.y / 16, chunk.size.z);
            }
        }

        c.queue.submit(Some(encoder.finish()));
    }
    pub fn resize(
        &self,
        device: &wgpu::Device,
        new_size: PhysicalSize<u32>,
        copy_pipeline: &wgpu::RenderPipeline
    ) {
        *self.output.lock().unwrap() = Output::new(
            device, new_size.width, new_size.height, &self.pipeline, copy_pipeline
        );
    }
}