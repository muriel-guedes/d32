#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Chunk {
    pub position: [f32;4],
    pub data: [[[u32;16];16];16]
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            position: [0.;4],
            data: [[[0;16];16];16]
        }
    }
}