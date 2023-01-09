use std::sync::Arc;

use crate::Chunks;

#[derive(Clone)]
pub struct Object {
    pub chunks: Arc<Chunks>
}
impl Object {
    pub fn new(
        device: &wgpu::Device,
        raster_pipeline: &wgpu::ComputePipeline,
        chunks: usize
    ) -> Self {
        Self {
            chunks: Arc::new(Chunks::new(device, raster_pipeline, chunks))
        }
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        self.chunks.update(queue)
    }
}