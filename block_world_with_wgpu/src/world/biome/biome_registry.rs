use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::world::biome::biome_type::BiomeType;
use crate::world::chunk::generator::ChunkGenerator;
use crate::block::block_registry::BlockRegistry;
use noise::OpenSimplex;
use crate::block::block_type::BlockType;
use crate::world::chunk::chunk::ChunkLocation;
use crate::block::location::BlockLocation;
use rand::{thread_rng, Rng};

pub struct BiomeRegistry {
    pub biomes: HashMap<String,Arc<BiomeType>>
}

impl BiomeRegistry {

    pub fn new() -> Self {
        Self { biomes: HashMap::new()}
    }

    pub fn register(&mut self,t:BiomeType) {
        self.biomes.insert(t.id.clone(),Arc::new(t));
    }

    pub fn get_biome(&self,id:String) -> Arc<BiomeType>{
        return self.biomes[&id].clone();
    }

    pub fn get_biome_for_range(&self,value:f64) -> Option<Arc<BiomeType>> {
        for (id,b) in self.biomes.iter() {
            if b.start_range <= value && b.end_range >= value {
                return Some(b.clone());
            }
        }

        None
    }

    pub fn add_all_types(&mut self) {
        self.register(BiomeType {
            id: "grass".to_string(),
            start_range: 0.0,
            end_range: 4.0,
            generator: Arc::new((BasicChunkGenerator {}))
        });
        self.register(BiomeType {
            id: "sand".to_string(),
            start_range: 4.0,
            end_range: 5.0,
            generator: Arc::new((SandChunkGenerator {}))
        });
        self.register(BiomeType {
            id: "snow".to_string(),
            start_range: 5.0,
            end_range: 8.0,
            generator: Arc::new((SnowGenerator {}))
        })
    }
}

struct BasicChunkGenerator {

}

impl ChunkGenerator for BasicChunkGenerator {
    fn generate_chunk_block(&self, block_registry: Arc<Mutex<BlockRegistry>>, noise: Arc<OpenSimplex>, loc: &ChunkLocation,world_block_loc:&BlockLocation,normal_block_loc:&BlockLocation, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>) {
        let y = self.sum_octave(16,world_block_loc.x as f64,world_block_loc.z as f64,0.5,0.01,1.0,crate::world::chunk::chunk::HEIGHT as f64,&noise) as i32;

        let t = block_registry.lock().unwrap().blocks["grass"].clone();

        for i in 0..y {
            blocks_out[i as usize][normal_block_loc.x as usize][normal_block_loc.z as usize] = t.clone();
        }
    }

    fn populate(&self, block_type: Arc<BlockType>, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>, loc: BlockLocation) {

    }
}

struct SandChunkGenerator {

}

impl ChunkGenerator for SandChunkGenerator {
    fn generate_chunk_block(&self, block_registry: Arc<Mutex<BlockRegistry>>, noise: Arc<OpenSimplex>, loc: &ChunkLocation,world_block_loc:&BlockLocation,normal_block_loc:&BlockLocation, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>) {
        let mut y = self.sum_octave(16,world_block_loc.x as f64,world_block_loc.z as f64,0.5,0.01,1.0,crate::world::chunk::chunk::HEIGHT as f64,&noise) as i32;


        let t = block_registry.lock().unwrap().blocks["sand"].clone();
        let log = block_registry.lock().unwrap().blocks["log"].clone();

        for i in 0..y {
            blocks_out[i as usize][normal_block_loc.x as usize][normal_block_loc.z as usize] = t.clone();
            if i == y-1 {
                if thread_rng().gen_range(0..20) == 19 {
                    blocks_out[i as usize + 1][normal_block_loc.x as usize][normal_block_loc.z as usize] = log.clone();
                }
            }
        }
    }

    fn populate(&self, block_type: Arc<BlockType>, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>, loc: BlockLocation) {

    }
}

struct SnowGenerator {

}

impl ChunkGenerator for SnowGenerator {
    fn generate_chunk_block(&self, block_registry: Arc<Mutex<BlockRegistry>>, noise: Arc<OpenSimplex>, loc: &ChunkLocation,world_block_loc:&BlockLocation,normal_block_loc:&BlockLocation, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>) {
        let y = self.sum_octave(16,world_block_loc.x as f64,world_block_loc.z as f64,0.5,0.01,1.0,crate::world::chunk::chunk::HEIGHT as f64,&noise) as i32;

        let t = block_registry.lock().unwrap().blocks["snow"].clone();

        for i in 0..y {
            blocks_out[i as usize][normal_block_loc.x as usize][normal_block_loc.z as usize] = t.clone();
        }
    }

    fn populate(&self, block_type: Arc<BlockType>, blocks_out: &mut Vec<Vec<Vec<Arc<BlockType>>>>, loc: BlockLocation) {

    }
}