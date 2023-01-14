use std::sync::Mutex;

use cgmath::Vector3;
use noise::{SuperSimplex, NoiseFn};

use crate::{Context, Chunk, Color};

pub struct Terrain {
    pub chunks: Mutex<Vec<Chunk>>
}
impl Terrain {
    pub fn new(c: &Context) -> Self {
        let size = Vector3::new(16, 16, 16);
        let noise_fn = SuperSimplex::new(0);

        let data_length = size.x + (size.y * size.x) + (size.z * size.y * size.x);
        let mut data = Vec::with_capacity(data_length as usize);
        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    let mut noise = noise_fn.get([x as f64, y as f64, z as f64]);
                    if noise < 0.0 { noise = 0. }
                    data.push(Color::from((
                        (noise * 255.) as u8,
                        (noise * 255.) as u8,
                        (noise * 255.) as u8,
                        (noise * 255.) as u8
                    )).get())
                }
            }
        }
        let position = Vector3::new(size.x as f32 / -2., size.y as f32 / -2., size.z as f32 / -2.);
        let chunk = Chunk::new(&c, data, position, size);
        
        Self {
            chunks: Mutex::new(vec![chunk])
        }
    }
    pub fn delete(self, c: &Context) {
        for chunk in self.chunks.lock().unwrap().iter() {
            c.remove_chunk(chunk)
        }
    }
}