use std::borrow::Cow;

use crate::Context;

pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("RayTraceShader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl")))
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("RayTraceShader pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[]
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: None,
                write_mask: wgpu::ColorWrites::COLOR
            })]
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview: None
    })
}

pub fn draw(c: &Context) {
    let mut encoder = c.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let output_texture = match c.surface.get_current_texture() {
        Ok(v) => v,
        Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => return c.resize(c.window.inner_size()),
        Err(e) => panic!("Error getting current surface texture: {}", e)
    };
    let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let chunks_bind_group = &c.chunks.bind_group.lock().unwrap();

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true
                }
            })],
            depth_stencil_attachment: None
        });
        render_pass.set_pipeline(&c.shader);
        render_pass.set_bind_group(0, &c.camera.bind_group, &[]);
        render_pass.set_bind_group(1, chunks_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    c.queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();
}