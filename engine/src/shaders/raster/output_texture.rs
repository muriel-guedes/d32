use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub width: f32,
    pub height: f32,
    pub width_i32: i32,
    pub height_i32: i32
}

pub struct Output {
    pub raster_bind_group: wgpu::BindGroup,
    pub copy_bind_group: wgpu::BindGroup,
    pub depth_buffer: wgpu::Buffer,
    pub depth_buffer_clear_data: Vec<f32>,
    pub output_buffer: wgpu::Buffer,
    pub output_buffer_clear_data: Vec<u32>
}

impl Output {
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

        let output_buffer_clear_data = vec![0u32;(width * height)as usize];
        let output_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&output_buffer_clear_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
        });

        let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[Size {
                width: width as f32,
                height: height as f32,
                width_i32: width as i32,
                height_i32: height as i32
            }]),
            usage: wgpu::BufferUsages::STORAGE
        });

        let raster_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &raster_pipeline.get_bind_group_layout(1),
            entries: &vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: output_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: depth_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: size_buffer.as_entire_binding()
                }
            ]
        });
        let copy_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &copy_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: output_buffer.as_entire_binding()
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: size_buffer.as_entire_binding()
                }
            ]
        });
        Self {
            raster_bind_group,
            copy_bind_group,
            depth_buffer,
            depth_buffer_clear_data,
            output_buffer,
            output_buffer_clear_data
        }
    }
}