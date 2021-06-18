use crate::block::block_type::BlockType;
use std::sync::{Arc, Mutex};
use crate::block::block_sides::BlockSides;
use nalgebra_glm::{TMat4, TVec3, vec3};
use crate::block::block_registry::BlockRegistry;
use std::path::Path;
use binary_rw::{BinaryReader, BinaryWriter};
use crate::world::chunk::chunk_mesh::ChunkMesh;
use crate::block::location::BlockLocation;
use crate::player::player::Player;
use rust_game_library_wgpu::objects::aabb::AABB;
use crate::data::save::Save;
use rust_game_library_wgpu::objects::camera::Camera;
use wgpu::Queue;
use crate::world::chunk::chunk::ChunkLocation;
use binary_rw::filestream::{Filestream, OpenType};

pub static WIDTH : usize = 16;
pub static HEIGHT : usize = 16;
pub static DEPTH : usize = 16;

#[derive(Clone)]
pub struct RenderChunk {
    pub location: RenderChunkLocation,
    pub blocks: Vec<Vec<Vec<Arc<BlockType>>>>,
    pub aabbs: Vec<AABB>,
    pub block_sides: Arc<BlockSides>,
    pub aabb: AABB,
}

#[derive(Clone,PartialEq, Eq, Hash)]
pub struct RenderChunkLocation {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl RenderChunkLocation {
    pub fn vec3(&self) -> TVec3<f32> {
        return vec3(self.x as f32,self.y as f32 * HEIGHT as f32, self.z as f32);
    }

    pub fn equals(&self, other:&RenderChunkLocation) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }

    pub fn new(x:i32,y:i32,z:i32) -> RenderChunkLocation {
        return RenderChunkLocation {
            x,
            y,
            z
        };
    }

    pub fn north(&self) -> RenderChunkLocation {
        return RenderChunkLocation::new(self.x, self.y,self.z+1);
    }

    pub fn east(&self) -> RenderChunkLocation {
        return RenderChunkLocation::new(self.x + 1,self.y, self.z);
    }

    pub fn south(&self) -> RenderChunkLocation {
        return RenderChunkLocation::new(self.x, self.y,self.z-1);
    }

    pub fn west(&self) -> RenderChunkLocation {
        return RenderChunkLocation::new(self.x - 1, self.y,self.z);
    }

    pub fn get_surrounding_chunks(&self) -> Vec<RenderChunkLocation> {
        return vec![self.north(),self.east(),self.south(),self.west()];
    }

    pub fn get_chunk_location(&self) -> ChunkLocation {
        return ChunkLocation::new(self.x,self.z);
    }
}

impl RenderChunk {
    pub fn new(blocks:Vec<Vec<Vec<Arc<BlockType>>>>,working_dir:String, location: RenderChunkLocation, block_registry:Arc<Mutex<BlockRegistry>>, block_sides: Arc<BlockSides>) -> RenderChunk {
        if Path::new(&format!("{}\\{}\\{}",working_dir,"world\\chunks",format!("c{}T{}T{}.dat",location.x,location.y,location.z))).exists() {
            let mut c = RenderChunk {
                location: location.clone(),
                blocks: vec![],
                aabbs: Vec::new(),
                block_sides: block_sides.clone(),
                aabb: AABB { min: vec3(location.x as f64 * WIDTH as f64 , 0.0, location.z as f64 * DEPTH as f64 ), max:  vec3(location.x as f64 * WIDTH as f64  + WIDTH as f64, 0.0, location.z as f64 * DEPTH as f64  + DEPTH as f64)},
            };

            c.load(format!("{}\\{}\\{}",working_dir,"world\\chunks",format!("c{}T{}T{}.dat",location.x,location.y,location.z)),block_registry);

            return c;
        }

        return RenderChunk {
            location: location.clone(),
            blocks,
            aabbs: Vec::new(),
            block_sides,
            aabb: AABB { min: vec3(location.x as f64 * WIDTH as f64 , 0.0, location.z as f64 * DEPTH as f64 ), max:  vec3(location.x as f64 * WIDTH as f64  + WIDTH as f64, 0.0, location.z as f64 * DEPTH as f64  + DEPTH as f64)},
        }
    }

    pub fn is_surrounded_by_solid(&self, loc:&BlockLocation) -> bool {
        return self.is_air(&loc.up()) && self.is_air(&loc.down()) && self.is_air(&loc.north()) && self.is_air(&loc.east()) && self.is_air(&loc.south()) && self.is_air(&loc.west());
    }

    pub fn is_air(&self, loc: &BlockLocation) -> bool{
        if loc.x > -1 && loc.y > -1 && loc.z > -1 && loc.x < WIDTH as i32 && loc.y < HEIGHT as i32 && loc.z < DEPTH as i32 {
            let t : Arc<BlockType> = self.blocks[loc.y as usize][loc.x as usize][loc.z as usize].clone();
            return !t.is_air;
        }
        return false;
    }

    pub fn is_transparent_and_self(&self, t:Arc<BlockType>,loc:&BlockLocation) -> bool{
        let b = self.get_block_at(loc);

        if b.is_some() {
            let a = b.unwrap();
            return a.is_transparent && a.is_air && t.is_transparent || a.is_transparent && !t.is_transparent;
        }

        return true;
    }

    pub fn is_air_unsafe(&self, loc: &BlockLocation) -> bool{
        let t : Arc<BlockType> = self.blocks[loc.y as usize][loc.x as usize][loc.z as usize].clone();
        return !t.is_air;
    }

    pub fn block_exists_in_chunk(&self,loc:&BlockLocation) -> bool {
        if loc.x > -1 && loc.y > -1 && loc.z > -1 && loc.x < WIDTH as i32 && loc.y < HEIGHT as i32 && loc.z < DEPTH as i32 {
            return true;
        }

        return false;
    }

    pub fn get_block_at(&self, loc: &BlockLocation) -> Option<Arc<BlockType>>{
        if loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x >= WIDTH as i32 || loc.y >= HEIGHT as i32 || loc.z >= DEPTH as i32 {
            return Option::None;
        }
        return Option::Some(self.blocks[loc.y as usize][loc.x as usize][loc.z as usize].clone());
    }

    pub unsafe fn set_block_at(&mut self, t: Arc<BlockType>, loc: &BlockLocation){
        if loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x >= WIDTH as i32 || loc.y >= HEIGHT as i32 || loc.z >= DEPTH as i32 {
            return;
        }

        self.blocks[loc.y as usize][loc.x as usize][loc.z as usize] = t.clone();
    }

    fn load(&mut self, file_path: String,block_registry:Arc<Mutex<BlockRegistry>>) {
        let mut file_stream = Filestream::new(file_path.as_str(),OpenType::Open).unwrap();
        let mut binary = BinaryReader::new(&mut file_stream);
        let size = binary.read_i16().unwrap();

        let mut blocks: Vec<Vec<Vec<Arc<BlockType>>>> = vec![vec![vec![block_registry.lock().unwrap().blocks["air"].clone();WIDTH];HEIGHT];DEPTH];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                for z in 0..DEPTH {
                    blocks[x][y][z] = block_registry.lock().unwrap().blocks[&binary.read_string().unwrap()].clone();
                }
            }
        }

        self.blocks = blocks;
    }

    pub fn save_real(&self,working_dir:String) {
        self.save(format!("{}\\{}\\{}",working_dir,"world\\chunks",format!("c{}T{}T{}.dat",self.location.x,self.location.y,self.location.z)));
    }
}

impl Save for RenderChunk {
    fn save(&self, file_path: String) {
        let mut file_stream = Filestream::new(file_path.as_str(),OpenType::OpenAndCreate).unwrap();
        let mut binary = BinaryWriter::new(&mut file_stream);
        binary.write_i16((HEIGHT * DEPTH * WIDTH) as i16);
        for x in self.blocks.iter() {
            for y in x.iter() {
                for block in y.iter() {
                    binary.write_string(block.id.to_string());
                }
            }
        }
    }
}