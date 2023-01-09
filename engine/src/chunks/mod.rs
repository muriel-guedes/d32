mod chunk;  pub use chunk::*;

pub struct Chunks {
    pub chunks: Vec<Chunk>
}
impl Chunks {
    pub fn new(
        device: &wgpu::Device,
        raster_pipeline: &wgpu::ComputePipeline,
        size: usize
    ) -> Self {
        let mut chunks = Vec::with_capacity(size);
        for _ in 0..size {
            chunks.push(Chunk::new(device, raster_pipeline))
        }
        Self {
            chunks
        }
    }
    pub fn update(&self, queue: &wgpu::Queue) {
        for chunk in self.chunks.iter() {
            chunk.update(queue)
        }
    }
}