use std::{borrow::Cow, sync::Mutex};
use winit::{window::Window, dpi::PhysicalSize};

use crate::Context;

mod output_texture;  pub use output_texture::*;

pub struct RasterShader {
    pub pipeline: wgpu::ComputePipeline,
    pub output_texture: Mutex<OutputTexture>
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
        let output_texture = OutputTexture::new(device, ws.width, ws.height, &pipeline, copy_pipeline);
        Self {
            pipeline,
            output_texture: Mutex::new(output_texture)
        }
    }
    pub fn draw(c: &Context) {
        let output_texture = c.raster_shader.output_texture.lock().unwrap();
        c.queue.write_buffer(&output_texture.depth_buffer, 0, bytemuck::cast_slice(
            &output_texture.depth_buffer_clear_data
        ));
        let output_texture = {
            c.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &output_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All
                },
                &output_texture.clear_data.lock().unwrap(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(4 * output_texture.size.width),
                    rows_per_image: std::num::NonZeroU32::new(output_texture.size.height)
                },
                output_texture.size
            );
            &output_texture.raster_bind_group
        };

        let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let objects = c.objects.lock().unwrap();

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&c.raster_shader.pipeline);
            cpass.set_bind_group(1, output_texture, &[]);
            cpass.set_bind_group(2, &c.camera.bind_group, &[]);
            for object in objects.iter() {
                for chunk in object.chunks.chunks.iter() {
                    cpass.set_bind_group(0, &chunk.bind_group, &[]);
                    cpass.dispatch_workgroups(1, 1, 16);
                }
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
        *self.output_texture.lock().unwrap() = OutputTexture::new(
            device, new_size.width, new_size.height, &self.pipeline, copy_pipeline
        );
    }
}