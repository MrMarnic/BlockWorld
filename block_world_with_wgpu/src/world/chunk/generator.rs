use crate::block::block_registry::BlockRegistry;
use std::sync::{Mutex, Arc};
use noise::{OpenSimplex, NoiseFn};
use crate::world::chunk::chunk::ChunkLocation;
use crate::block::block_type::BlockType;
use crate::block::location::BlockLocation;

pub trait ChunkGenerator {
    fn generate_chunk_block(&self,block_registry:Arc<Mutex<BlockRegistry>>,noise:Arc<OpenSimplex>,loc:&ChunkLocation,world_block_loc:&BlockLocation,normal_block_loc:&BlockLocation, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>);
    fn sum_octave(&self,num_iterations:i32,x:f64,y:f64,persistence:f64,scale:f64,low:f64,high:f64,noise_fn:&OpenSimplex) -> f64{
        let mut max_amp = 0.0;
        let mut amp = 1.0;
        let mut freq = scale;
        let mut noise = 0.0;

        for i in 0..num_iterations {
            noise += noise_fn.get([x * freq,y * freq]) * amp;
            max_amp += amp;
            amp *= persistence;
            freq *= 2.0;
        }

        noise /= max_amp;

        noise = noise * (high-low) / 2.0 + (high + low) / 2.0;

        return noise;
    }
    fn populate(&self,block_type:Arc<BlockType>, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>,loc:BlockLocation);
}