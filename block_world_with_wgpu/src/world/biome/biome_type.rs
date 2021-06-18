use crate::world::chunk::generator::ChunkGenerator;
use std::sync::Arc;

pub struct BiomeType {
    pub id: String,
    pub start_range: f64,
    pub end_range: f64,
    pub generator : Arc<dyn ChunkGenerator + Sync + Send>
}

impl BiomeType {
    pub fn new(id:String, start:f64, end:f64, generator: Arc<dyn ChunkGenerator + Sync + Send>) -> Self {
        Self {
            id,
            start_range: start,
            end_range: end,
            generator
        }
    }
}