use std::sync::Mutex;

use crate::{Context, Chunk};

pub struct Terrain {
    pub chunks: Mutex<Vec<Chunk>>
}
impl Terrain {
    pub fn new(c: &Context) -> Self {
        let mut chunks = Vec::new();
        chunks.push(Chunk::new_gradient_filled_cube(c, [-32.;3].into(), [64;3].into()));
        for chunk in chunks.iter() {
            c.add_chunk(chunk.clone());
        }
        Self {
            chunks: Mutex::new(chunks)
        }
    }
    pub fn delete(self, c: &Context) {
        for chunk in self.chunks.lock().unwrap().iter() {
            c.remove_chunk(chunk)
        }
    }
}